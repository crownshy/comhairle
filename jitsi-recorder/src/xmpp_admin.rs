//! A minimal, purpose-built XMPP-over-WebSocket client (RFC 7395) for exactly one
//! operation: granting a MUC affiliation as an admin account. gst-meet is only ever
//! used as a spawned subprocess (see `start_gst_meet_process`) and has no mechanism
//! to send an arbitrary IQ before joining, so this exists to do that one thing.
//!
//! Why this is needed: breakout-room joins authenticate as the admin-listed
//! `recorder@hidden.meet.jitsi` account instead of a bot JWT (see
//! `StartRecordingRequest::xmpp_username`). Being a Prosody admin is enough to get
//! past `mod_muc_breakout_rooms.lua`'s join check, but jicofo separately requires a
//! real MUC affiliation (member/owner, not "none") before it will send a Jingle
//! session-initiate - confirmed live via a manual XMPP trace: with `affiliation='none'`
//! the join succeeds but jicofo never invites the occupant into media; granting
//! `member` fixed it immediately (full session-initiate/session-accept exchange).
//! `mod_muc_breakout_rooms.lua` proxies a breakout room's `get_affiliation` straight
//! to the corresponding main room's affiliation table, so the grant has to happen on
//! the main room, not the breakout room itself.

use anyhow::{bail, Context, Result};
use base64::Engine;
use futures_util::{SinkExt, Stream, StreamExt};
use tokio::time::{timeout, Duration};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;
use uuid::Uuid;

const STANZA_TIMEOUT: Duration = Duration::from_secs(10);

/// Authenticates as `username`/`password` on `auth_domain` and grants `affiliation`
/// (e.g. "member") to `affiliate_jid` on `room_jid`, then disconnects.
pub async fn grant_muc_affiliation(
    websocket_url: &str,
    auth_domain: &str,
    username: &str,
    password: &str,
    room_jid: &str,
    affiliate_jid: &str,
    affiliation: &str,
) -> Result<()> {
    let mut request = websocket_url
        .into_client_request()
        .context("invalid websocket URL")?;
    request
        .headers_mut()
        .insert("Sec-WebSocket-Protocol", "xmpp".parse()?);

    let (ws_stream, _) = timeout(STANZA_TIMEOUT, tokio_tungstenite::connect_async(request))
        .await
        .context("timed out connecting to XMPP websocket")?
        .context("failed to connect to XMPP websocket")?;
    let (mut write, mut read) = ws_stream.split();

    write
        .send(Message::Text(format!(
            "<open xmlns='urn:ietf:params:xml:ns:xmpp-framing' to=\"{auth_domain}\" version=\"1.0\"/>"
        )))
        .await?;
    recv_containing(&mut read, &["mechanism"]).await?;

    let auth_payload =
        base64::engine::general_purpose::STANDARD.encode(format!("\0{username}\0{password}"));
    write
        .send(Message::Text(format!(
            "<auth xmlns='urn:ietf:params:xml:ns:xmpp-sasl' mechanism=\"PLAIN\">{auth_payload}</auth>"
        )))
        .await?;
    let (matched, sasl_response) = recv_containing(&mut read, &["success", "failure"]).await?;
    if matched == "failure" {
        bail!("SASL PLAIN authentication failed: {sasl_response}");
    }

    write
        .send(Message::Text(format!(
            "<open xmlns='urn:ietf:params:xml:ns:xmpp-framing' to=\"{auth_domain}\" version=\"1.0\"/>"
        )))
        .await?;
    recv_containing(&mut read, &["bind"]).await?;

    let bind_id = Uuid::new_v4();
    write
        .send(Message::Text(format!(
            "<iq xmlns='jabber:client' id=\"{bind_id}\" type=\"set\"><bind xmlns='urn:ietf:params:xml:ns:xmpp-bind'/></iq>"
        )))
        .await?;
    recv_containing(&mut read, &[&bind_id.to_string()]).await?;

    let admin_id = Uuid::new_v4();
    write
        .send(Message::Text(format!(
            "<iq xmlns='jabber:client' id=\"{admin_id}\" to=\"{room_jid}\" type=\"set\"><query xmlns='http://jabber.org/protocol/muc#admin'><item affiliation=\"{affiliation}\" jid=\"{affiliate_jid}\"/></query></iq>"
        )))
        .await?;
    let (_, admin_response) = recv_containing(&mut read, &[&admin_id.to_string()]).await?;
    if admin_response.contains("type='error'") || admin_response.contains("type=\"error\"") {
        bail!("MUC affiliation grant rejected: {admin_response}");
    }

    info!("Granted {affiliation} affiliation to {affiliate_jid} on {room_jid}");

    let _ = write
        .send(Message::Text(
            "<close xmlns='urn:ietf:params:xml:ns:xmpp-framing'/>".to_string(),
        ))
        .await;

    Ok(())
}

/// Waits for a text frame containing any of `needles`, returning which one matched
/// (by value, for simple equality checks) and the full frame text.
async fn recv_containing<S>(read: &mut S, needles: &[&str]) -> Result<(String, String)>
where
    S: Stream<Item = std::result::Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    loop {
        let msg = timeout(STANZA_TIMEOUT, read.next())
            .await
            .context("timed out waiting for XMPP response")?
            .context("XMPP websocket closed unexpectedly")?
            .context("XMPP websocket error")?;
        if let Message::Text(text) = msg {
            if let Some(needle) = needles.iter().find(|n| text.contains(**n)) {
                return Ok(((*needle).to_string(), text));
            }
        }
    }
}
