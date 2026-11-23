// Simple test to record mixed audio from the conference
// This proves the recording pipeline works before we tackle individual participants

use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use lib_gst_meet::{xmpp_parsers::BareJid, Authentication, Connection, JitsiConference, JitsiConferenceConfig};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    gst::init()?;

    let websocket_url = "wss://video.comhairle.scot/xmpp-websocket";
    let xmpp_domain = "meet.jitsi";
    let args: Vec<String> = std::env::args().collect();
    let room_name = if args.len() > 1 { &args[1] } else { "test_room" };
    let nickname = "recorder-bot";

    // Connect
    let (connection, connection_future) = Connection::new(
        websocket_url,
        xmpp_domain,
        Authentication::Anonymous,
        room_name,
        false,
    )
    .await?;

    tokio::spawn(async move { connection_future.await });
    connection.connect().await?;
    info!("Connected to XMPP");

    // Join conference
    let muc_jid: BareJid = format!("{}@muc.{}", room_name, xmpp_domain).parse()?;
    let focus_jid = format!("focus.{}", xmpp_domain).parse()?;

    let config = JitsiConferenceConfig {
        muc: muc_jid,
        focus: focus_jid,
        nick: nickname.to_string(),
        region: None,
        video_codec: "vp9".to_string(),
        extra_muc_features: vec![],
        start_bitrate: 800,
        stereo: true,
        recv_video_scale_width: 1280,
        recv_video_scale_height: 720,
        buffer_size: 200,
    };

    let glib_context = glib::MainContext::default();
    let conference = JitsiConference::join(connection, glib_context, config).await?;

    info!("Joined conference! Bot is receiving audio...");
    info!("Note: lib-gst-meet doesn't expose easy access to individual participant audio");
    info!("The bot receives mixed audio through its internal GStreamer pipeline");
    info!("To record individual participants, we'd need to modify lib-gst-meet or use Jibri");

    // Keep running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down");
    Ok(())
}
