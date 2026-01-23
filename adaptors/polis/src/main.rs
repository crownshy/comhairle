mod polis;
use std::{collections::HashMap, error::Error};

use polis::{self, PolisConnector, User};
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let poll_id = 12;
    let polis = PolisConnector::new(
        "postgres://postgres:polis@localhost:5431/polis-dev",
        "https://communitypolis.crown-shy.com",
    )
    .await?;

    let comments = polis.get_comments(poll_id).await?;
    let participants = polis.get_participants(poll_id).await?;
    let users = polis.get_active_users(poll_id).await?;
    let math = polis.get_math(poll_id).await?;

    // println!("{math:#?}");
    let votes = polis.get_votes(poll_id).await?;

    let xids = polis.get_xids().await?;

    let uids: Vec<i32> = xids.iter().map(|p| p.uid).collect();
    let xids: Vec<String> = xids.iter().map(|p| p.xid.clone()).collect();

    let uids_to_xids: HashMap<i32, String> = uids.into_iter().zip(xids.into_iter()).collect();

    let uids: Vec<i32> = participants.iter().map(|p| p.uid).collect();
    let pids: Vec<i32> = participants.iter().map(|p| p.pid).collect();

    let pid_to_uid: HashMap<i32, i32> = pids.into_iter().zip(uids.into_iter()).collect();

    println!("{pid_to_uid:#?} {}", pid_to_uid.len());

    let groups = polis.get_group_membership(poll_id).await?;

    let ids: Vec<i32> = comments.iter().map(|c| c.uid).collect();
    let conversation_users: Vec<&User> = users.iter().filter(|u| ids.contains(&u.uid)).collect();

    let mut file = File::create("votes.csv")?;
    writeln!(file, "comment_id,vote,x_id")?;
    for vote in votes {
        if let Some(uid) = pid_to_uid.get(&vote.pid) {
            if let Some(xid) = uids_to_xids.get(uid) {
                writeln!(file, "{},{},{}", vote.tid, vote.vote, xid)?;
            };
        };
    }

    let mut file = File::create("comments.csv")?;
    writeln!(file, "comment_id,text")?;
    for comment in comments {
        writeln!(file, "{},\"{}\"", comment.tid, comment.txt)?;
    }

    let (pid_to_uid, _, pids_to_xids) = polis.get_lookups(poll_id).await?;

    let mut file = File::create("crosswalk.csv")?;
    writeln!(file, "pid,uid,xid")?;
    for (pid, uid) in pid_to_uid.iter() {
        if let Some(xid) = pids_to_xids.get(pid) {
            writeln!(file, "{pid},{uid},\"{xid}\"")?;
        }
    }

    Ok(())
}

