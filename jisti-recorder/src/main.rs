use anyhow::{anyhow, Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use lib_gst_meet::{xmpp_parsers::BareJid, Authentication, Connection, JitsiConference, JitsiConferenceConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, warn};

/// Creates a GStreamer bin for recording a participant's audio
async fn create_recording_bin(
    conference: &JitsiConference,
    participant_id: &str,
    filename: &str,
) -> Result<gst::Bin> {
    info!("Creating recording bin for participant: {}", participant_id);

    // Create a bin to hold the recording pipeline
    let bin = gst::Bin::new(Some(&format!("recording-{}", participant_id)));

    // Create GStreamer elements for the recording pipeline
    let queue = gst::ElementFactory::make("queue")
        .name(&format!("queue-{}", participant_id))
        .build()
        .context("Failed to create queue element")?;

    let audioconvert = gst::ElementFactory::make("audioconvert")
        .name(&format!("audioconvert-{}", participant_id))
        .build()
        .context("Failed to create audioconvert element")?;

    let audioresample = gst::ElementFactory::make("audioresample")
        .name(&format!("audioresample-{}", participant_id))
        .build()
        .context("Failed to create audioresample element")?;

    let encoder = gst::ElementFactory::make("wavenc")
        .name(&format!("wavenc-{}", participant_id))
        .build()
        .context("Failed to create wavenc element")?;

    let filesink = gst::ElementFactory::make("filesink")
        .name(&format!("filesink-{}", participant_id))
        .property("location", filename)
        .build()
        .context("Failed to create filesink element")?;

    // Add elements to the bin
    bin.add_many(&[&queue, &audioconvert, &audioresample, &encoder, &filesink])
        .context("Failed to add elements to bin")?;

    // Link elements together
    gst::Element::link_many(&[&queue, &audioconvert, &audioresample, &encoder, &filesink])
        .context("Failed to link elements")?;

    // Create a ghost pad named "audio" on the bin for the queue's sink pad
    // Note: lib-gst-meet may automatically route participant audio to elements/pads named "audio"
    let queue_sink = queue.static_pad("sink")
        .ok_or_else(|| anyhow!("Failed to get queue sink pad"))?;
    let ghost_pad = gst::GhostPad::with_target(Some("audio"), &queue_sink)
        .context("Failed to create ghost pad")?;
    ghost_pad.set_active(true)
        .context("Failed to activate ghost pad")?;
    bin.add_pad(&ghost_pad)
        .context("Failed to add ghost pad to bin")?;

    // Add the bin to the conference pipeline
    conference.add_bin(&bin).await
        .context("Failed to add bin to conference pipeline")?;

    // Set the bin to playing state
    bin.set_state(gst::State::Playing)
        .context("Failed to set bin to playing state")?;

    info!("Recording bin created and added to pipeline for: {}", participant_id);

    Ok(bin)
}

/// Cleans up a recording bin
async fn cleanup_recording_bin(bin: gst::Bin) -> Result<()> {
    info!("Cleaning up recording bin: {:?}", bin.name());

    // Send EOS to flush the pipeline
    bin.send_event(gst::event::Eos::new());

    // Wait a bit for EOS to propagate
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Set bin to null state
    bin.set_state(gst::State::Null)
        .context("Failed to set bin to null state")?;

    info!("Recording bin cleaned up");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Initialize GStreamer
    gst::init()?;
    info!("GStreamer initialized");

    // 1. Configure connection parameters
    let websocket_url = "wss://video.comhairle.scot/xmpp-websocket";
    let xmpp_domain = "meet.jitsi";

    // Get room name from command line argument, or use default
    let args: Vec<String> = std::env::args().collect();
    let room_name = if args.len() > 1 {
        &args[1]
    } else {
        "test_room"
    };

    let nickname = "recorder-bot";

    // 2. Create XMPP connection (using anonymous authentication)
    let (connection, connection_future) = Connection::new(
        websocket_url,
        xmpp_domain,
        Authentication::Anonymous,
        room_name,
        false, // tls_insecure = false (verify certificates)
    )
    .await
    .context("Failed to create XMPP connection")?;

    // Spawn the connection future to handle the WebSocket
    tokio::spawn(async move {
        connection_future.await;
    });

    // Connect to XMPP server
    connection.connect().await.context("Failed to connect to XMPP server")?;
    info!("Connected to XMPP server");

    // 3. Create conference configuration
    let muc_jid: BareJid = format!("{}@muc.{}", room_name, xmpp_domain)
        .parse()
        .context("Failed to parse MUC JID")?;

    let focus_jid = format!("focus.{}", xmpp_domain)
        .parse()
        .context("Failed to parse focus JID")?;

    let conference_config = JitsiConferenceConfig {
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

    // 4. Get glib main context
    let glib_main_context = glib::MainContext::default();

    // 5. Join the conference
    info!("Joining conference room: {}", room_name);
    let conference = JitsiConference::join(
        connection,
        glib_main_context.clone(),
        conference_config,
    )
    .await
    .context("Failed to join conference")?;

    info!("Successfully joined conference! Waiting for participants...");

    // 6. Create a map to track participant recording bins
    let participant_bins: Arc<Mutex<HashMap<String, gst::Bin>>> = Arc::new(Mutex::new(HashMap::new()));

    // 7. Set up participant handler to record audio for each participant
    info!("Setting up participant join handler...");
    let bins_clone = participant_bins.clone();
    conference
        .on_participant(move |conference, participant| {
            let bins = bins_clone.clone();
            Box::pin(async move {
                info!("on_participant callback triggered!");
                let participant_id = participant.muc_jid.resource.to_string();
                let nick = participant.nick.as_deref().unwrap_or("unknown");

                info!(
                    "Participant joined: {} (nick: {})",
                    participant_id, nick
                );

                // Create filename for this participant's recording
                let filename = format!("recording_{}_{}.wav", participant_id, nick);
                info!("Will record to: {}", filename);

                // Create a GStreamer bin for recording this participant's audio
                match create_recording_bin(&conference, &participant_id, &filename).await {
                    Ok(bin) => {
                        bins.lock().await.insert(participant_id.clone(), bin);
                        info!("Successfully set up recording for participant: {}", participant_id);
                    }
                    Err(e) => {
                        error!("Failed to create recording bin for {}: {:?}", participant_id, e);
                    }
                }

                Ok(())
            })
        })
        .await;
    info!("Participant join handler registered");

    info!("Setting up participant leave handler...");
    let bins_clone = participant_bins.clone();
    conference
        .on_participant_left(move |_conference, participant| {
            let bins = bins_clone.clone();
            Box::pin(async move {
                info!("on_participant_left callback triggered!");
                let participant_id = participant.muc_jid.resource.to_string();
                info!(
                    "Participant left: {} (nick: {:?})",
                    participant_id, participant.nick
                );

                // Remove and cleanup the recording bin
                if let Some(bin) = bins.lock().await.remove(&participant_id) {
                    info!("Cleaning up recording for: {}", participant_id);
                    if let Err(e) = cleanup_recording_bin(bin).await {
                        warn!("Error cleaning up recording bin: {:?}", e);
                    }
                }

                Ok(())
            })
        })
        .await;
    info!("Participant leave handler registered");

    // 8. Keep the application running
    info!("Ready! Waiting for participants to join the room...");
    info!("Join the meeting at: https://video.comhairle.scot/{}", room_name);
    info!("Press Ctrl+C to exit");

    // Keep the main thread alive - in a real application you'd want proper shutdown handling
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    Ok(())
}
