#[cfg(test)]
use std::collections::HashMap;
use std::ops::Deref;
#[cfg(test)]
use std::pin::Pin;
#[cfg(test)]
use std::sync::{Arc, Mutex as StdMutex, PoisonError};
#[cfg(test)]
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures_util::StreamExt;
use futures_util::stream::BoxStream;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisError};
#[cfg(test)]
use tokio::sync::{Mutex, mpsc};

/// A subscription that owns its connection and yields raw message payloads.
/// Dropping the stream disconnects the subscriber.
pub type RedisSubscription = BoxStream<'static, Vec<u8>>;

/// Abstracts over Redis caching and channel-based Pub/Sub.
///
/// Both the production [`RedisImpl`] (backed by a real `ConnectionManager`)
/// and the test-only [`MockRedis`] (backed by an in-memory `HashMap`)
/// implement this trait.
#[async_trait]
pub trait RedisConnection: Send + Sync {
    /// Retrieve the value stored at `key`, or `None` if absent.
    async fn get(&self, key: &str) -> Result<Option<String>, RedisError>;

    /// Retrieve values for multiple keys in a single round-trip.
    /// Missing keys are represented as `None`.
    async fn get_multi(&self, keys: &[&str]) -> Result<Vec<Option<String>>, RedisError>;

    /// Store `value` at `key` with a TTL of `ttl_secs` seconds.
    async fn set_ex(&self, key: &str, value: &str, ttl_secs: u64) -> Result<(), RedisError>;

    /// Delete the entry at `key`.
    async fn del(&self, key: &str) -> Result<(), RedisError>;

    /// Publish a payload, returning the number of subscribers it was sent to.
    /// Messages are not retained for future subscribers.
    async fn publish(&self, channel: &str, payload: &[u8]) -> Result<usize, RedisError>;

    /// Subscribe to a channel. The subscription is ready when this returns.
    /// Every active subscriber receives its own copy, including the publisher.
    /// The stream ends if the subscription connection closes.
    async fn subscribe(&self, channel: &str) -> Result<RedisSubscription, RedisError>;
}

/// Production Redis connection backed by a [`redis::aio::ConnectionManager`].
#[derive(Debug)]
pub struct RedisImpl {
    connection: ConnectionManager,
    client: redis::Client,
}

impl RedisImpl {
    /// The client must point to the same Redis server as the connection manager.
    pub fn new(connection: ConnectionManager, client: redis::Client) -> Self {
        Self { connection, client }
    }
}

impl Deref for RedisImpl {
    type Target = ConnectionManager;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

#[async_trait]
impl RedisConnection for RedisImpl {
    async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut conn = (*self).clone();
        conn.get::<_, Option<String>>(key).await.map_err(|e| {
            tracing::error!("Redis get error for key {key}: {e}");
            e
        })
    }

    async fn get_multi(&self, keys: &[&str]) -> Result<Vec<Option<String>>, RedisError> {
        if keys.is_empty() {
            return Ok(vec![]);
        }
        let mut conn = (*self).clone();
        let mut pipe = redis::pipe();
        for key in keys {
            pipe.get(*key);
        }
        pipe.query_async::<Vec<Option<String>>>(&mut conn)
            .await
            .map_err(|e| {
                tracing::error!("Redis get_multi error: {e}");
                e
            })
    }

    async fn set_ex(&self, key: &str, value: &str, ttl_secs: u64) -> Result<(), RedisError> {
        let mut conn = (*self).clone();
        conn.set_ex::<_, _, ()>(key, value, ttl_secs)
            .await
            .map_err(|e| {
                tracing::error!("Redis set_ex error for key {key}: {e}");
                e
            })
    }

    async fn del(&self, key: &str) -> Result<(), RedisError> {
        let mut conn = (*self).clone();
        conn.del::<_, ()>(key).await.map_err(|e| {
            tracing::error!("Redis del error for key {key}: {e}");
            e
        })
    }

    async fn publish(&self, channel: &str, payload: &[u8]) -> Result<usize, RedisError> {
        let mut connection = self.connection.clone();
        connection.publish(channel, payload).await.map_err(|error| {
            tracing::error!("Redis publish error for channel {channel}: {error}");
            error
        })
    }

    async fn subscribe(&self, channel: &str) -> Result<RedisSubscription, RedisError> {
        let mut pubsub = self.client.get_async_pubsub().await.map_err(|error| {
            tracing::error!("Redis subscription connection error: {error}");
            error
        })?;
        pubsub.subscribe(channel).await.map_err(|error| {
            tracing::error!("Redis subscribe error for channel {channel}: {error}");
            error
        })?;
        Ok(pubsub
            .into_on_message()
            .map(|message| message.get_payload_bytes().to_vec())
            .boxed())
    }
}

/// In-memory Redis mock with an isolated cache and per-channel subscribers.
/// Share one instance between services to test fan-out without a Redis server.
#[cfg(test)]
#[derive(Default)]
pub struct MockRedis {
    values: Arc<Mutex<HashMap<String, String>>>,
    subscribers: Arc<StdMutex<MockSubscribers>>,
}

#[cfg(test)]
type MockSubscribers = HashMap<String, Vec<mpsc::UnboundedSender<Vec<u8>>>>;

#[cfg(test)]
struct MockRedisSubscription {
    receiver: mpsc::UnboundedReceiver<Vec<u8>>,
    sender: mpsc::UnboundedSender<Vec<u8>>,
    channel: String,
    subscribers: Arc<StdMutex<MockSubscribers>>,
}

#[cfg(test)]
impl futures_util::Stream for MockRedisSubscription {
    type Item = Vec<u8>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(context)
    }
}

#[cfg(test)]
impl Drop for MockRedisSubscription {
    fn drop(&mut self) {
        let mut channels = self
            .subscribers
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(subscribers) = channels.get_mut(&self.channel) {
            subscribers.retain(|subscriber| !subscriber.same_channel(&self.sender));
            if subscribers.is_empty() {
                channels.remove(&self.channel);
            }
        }
    }
}

#[cfg(test)]
impl MockRedis {
    pub fn new() -> Self {
        Self::default()
    }

    /// Directly read a value from the in-memory store.
    ///
    /// Useful in tests to assert that the cache has been populated or
    /// invalidated without going through the trait interface.
    pub async fn get_value(&self, key: &str) -> Option<String> {
        self.lock().await.get(key).cloned()
    }
}

#[cfg(test)]
impl Deref for MockRedis {
    type Target = Arc<Mutex<HashMap<String, String>>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[cfg(test)]
#[async_trait]
impl RedisConnection for MockRedis {
    async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        Ok(self.lock().await.get(key).cloned())
    }

    async fn get_multi(&self, keys: &[&str]) -> Result<Vec<Option<String>>, RedisError> {
        let store = self.lock().await;
        Ok(keys.iter().map(|k| store.get(*k).cloned()).collect())
    }

    async fn set_ex(&self, key: &str, value: &str, _ttl_secs: u64) -> Result<(), RedisError> {
        self.lock().await.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn del(&self, key: &str) -> Result<(), RedisError> {
        self.lock().await.remove(key);
        Ok(())
    }

    async fn publish(&self, channel: &str, payload: &[u8]) -> Result<usize, RedisError> {
        let mut channels = self
            .subscribers
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let Some(subscribers) = channels.get_mut(channel) else {
            return Ok(0);
        };
        subscribers.retain(|subscriber| subscriber.send(payload.to_vec()).is_ok());
        Ok(subscribers.len())
    }

    async fn subscribe(&self, channel: &str) -> Result<RedisSubscription, RedisError> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.subscribers
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .entry(channel.to_owned())
            .or_default()
            .push(sender.clone());
        Ok(MockRedisSubscription {
            receiver,
            sender,
            channel: channel.to_owned(),
            subscribers: self.subscribers.clone(),
        }
        .boxed())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures_util::FutureExt;
    use tokio::time::timeout;

    use super::*;

    #[tokio::test]
    async fn pubsub_fans_out_to_every_subscriber_in_order() -> Result<(), Box<dyn std::error::Error>>
    {
        let redis = MockRedis::new();
        let mut first = redis.subscribe("updates").await?;
        let mut second = redis.subscribe("updates").await?;
        let payloads: [&[u8]; 2] = [b"first", &[0, 255, 1]];

        for payload in payloads {
            assert_eq!(redis.publish("updates", payload).await?, 2);
        }
        for subscriber in [&mut first, &mut second] {
            for payload in payloads {
                assert_eq!(
                    timeout(Duration::from_secs(1), subscriber.next()).await?,
                    Some(payload.to_vec())
                );
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn pubsub_isolates_channels_and_mock_instances() -> Result<(), Box<dyn std::error::Error>>
    {
        let redis = MockRedis::new();
        let other_redis = MockRedis::new();
        let mut updates = redis.subscribe("updates").await?;
        let mut other_channel = redis.subscribe("other").await?;
        let mut other_instance = other_redis.subscribe("updates").await?;

        assert_eq!(redis.publish("updates", b"update").await?, 1);
        assert_eq!(
            timeout(Duration::from_secs(1), updates.next()).await?,
            Some(b"update".to_vec())
        );
        assert!(other_channel.next().now_or_never().is_none());
        assert!(other_instance.next().now_or_never().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn pubsub_does_not_replay_or_count_dropped_subscribers()
    -> Result<(), Box<dyn std::error::Error>> {
        let redis = MockRedis::new();
        assert_eq!(redis.publish("updates", b"before subscribing").await?, 0);

        let mut dropped = redis.subscribe("updates").await?;
        assert!(dropped.next().now_or_never().is_none());
        let mut active = redis.subscribe("updates").await?;
        drop(dropped);

        assert_eq!(redis.publish("updates", b"after subscribing").await?, 1);
        assert_eq!(
            timeout(Duration::from_secs(1), active.next()).await?,
            Some(b"after subscribing".to_vec())
        );
        assert!(active.next().now_or_never().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn dropping_subscriptions_removes_senders_and_empty_channels()
    -> Result<(), Box<dyn std::error::Error>> {
        let redis = MockRedis::new();
        let first = redis.subscribe("updates").await?;
        let second = redis.subscribe("updates").await?;
        let other = redis.subscribe("other").await?;

        drop(first);
        {
            let channels = redis
                .subscribers
                .lock()
                .unwrap_or_else(PoisonError::into_inner);
            assert_eq!(channels.get("updates").map(Vec::len), Some(1));
            assert_eq!(channels.get("other").map(Vec::len), Some(1));
        }
        drop(second);
        assert!(
            !redis
                .subscribers
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .contains_key("updates")
        );
        drop(other);
        assert!(
            redis
                .subscribers
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .is_empty()
        );
        Ok(())
    }
}
