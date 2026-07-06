#[cfg(test)]
use std::collections::HashMap;
use std::ops::Deref;
#[cfg(test)]
use std::sync::Arc;

use async_trait::async_trait;
use redis::AsyncCommands;
use redis::RedisError;
use redis::aio::ConnectionManager;
#[cfg(test)]
use tokio::sync::Mutex;

/// Abstracts over a Redis connection.
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
}

/// Production Redis connection backed by a [`redis::aio::ConnectionManager`].
#[derive(Debug)]
pub struct RedisImpl(ConnectionManager);

impl RedisImpl {
    pub fn new(conn: ConnectionManager) -> Self {
        Self(conn)
    }
}

impl Deref for RedisImpl {
    type Target = ConnectionManager;

    fn deref(&self) -> &Self::Target {
        &self.0
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
}

/// In-memory Redis mock backed by a `HashMap<String, String>`.
#[cfg(test)]
pub struct MockRedis(Arc<Mutex<HashMap<String, String>>>);

#[cfg(test)]
impl MockRedis {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
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
        &self.0
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
}
