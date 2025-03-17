use std::collections::HashMap;

use sea_query::{
    enum_def, ColumnDef, Expr, Iden, PostgresQueryBuilder, Query, SelectStatement, Table,
};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::{types::BigDecimal, FromRow};
use sqlx_postgres::{PgPool, PgPoolOptions};
use thiserror::Error;

#[derive(Debug, Deserialize, FromRow)]
#[enum_def(table_name = "users")]
struct User {
    uid: i32,
    username: Option<String>,
    email: Option<String>,
    site_owner: bool,
}

#[derive(Debug, Deserialize, FromRow)]
#[enum_def(table_name = "votes")]
struct Vote {
    pid: i32,
    zid: i32,
    vote: i16,
    tid: i32,
    high_priority: bool,
}

#[derive(Debug, Deserialize)]
struct VoteSummary {
    A: i32,
    D: i32,
    S: i32,
}

#[derive(Debug, Deserialize)]
struct GroupVotes {
    votes: HashMap<String, VoteSummary>,
    #[serde(rename = "n-members")]
    n_members: i32,
}

#[derive(Debug, Deserialize)]
struct GroupCluster {
    id: i32,
    center: [f32; 2],
    members: Vec<i32>,
}
#[derive(Debug, Deserialize)]
struct SubGroupCluster {
    id: i32,
    center: [f32; 2],
    members: Vec<i32>,
    #[serde(rename = "parent-id")]
    parent_id: i32,
}

#[derive(Debug, Deserialize)]
struct Pca {
    comps: Vec<Vec<f32>>,
    center: Vec<f32>,
    #[serde(rename = "comment-extremity")]
    comment_extremity: Vec<f32>,
    #[serde(rename = "comment-projection")]
    comment_projection: Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize)]
struct MathData {
    n: i32,
    pca: Pca,
    zid: i32,
    tids: Vec<i32>,
    #[serde(rename = "mod-in")]
    mod_in: Vec<i32>,
    #[serde(rename = "n-cmts")]
    n_cmts: i32,
    #[serde(rename = "in-conv")]
    in_conv: Vec<i32>,
    #[serde(rename = "mod-out")]
    mod_out: Vec<i32>,
    #[serde(rename = "group-votes")]
    group_votes: HashMap<String, GroupVotes>,
    #[serde(rename = "group-clusters")]
    group_clusters: Vec<GroupCluster>,
    #[serde(rename = "user-vote-counts")]
    user_vote_counts: HashMap<String, i32>,
    #[serde(rename = "subgroup-clusters")]
    subgroup_clusters: HashMap<String, Vec<SubGroupCluster>>,
    #[serde(rename = "comment-priorities")]
    comment_priorities: HashMap<String, f32>,
    #[serde(rename = "group-aware-consensus")]
    group_aware_consensus: HashMap<String, f32>,
}

#[derive(Debug, Deserialize, FromRow)]
#[enum_def(table_name = "math_main")]
struct Math {
    zid: i32,
    data: sqlx::types::Json<MathData>,
}

#[derive(Debug, Deserialize, FromRow)]
#[enum_def(table_name = "comments")]
struct Comment {
    pid: i32,
    zid: i32,
    uid: i32,
    txt: String,
    lang: Option<String>,
    anon: bool,
}

pub struct PolisConnector {
    db: PgPool,
    server_url: String,
}

#[derive(Error, Debug)]
pub enum PolisConnectionError {
    #[error("data store disconnected")]
    FailedToConnect(#[from] sqlx::Error),
}

impl PolisConnector {
    pub async fn new(
        db_connection_str: &str,
        server_url: &str,
    ) -> Result<Self, PolisConnectionError> {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_connection_str)
            .await?;
        return Ok(Self {
            db,
            server_url: server_url.into(),
        });
    }

    pub async fn get_active_users(&self, zid: i32) -> Result<Vec<User>, PolisConnectionError> {
        let (sql, values) = Query::select()
            .columns([
                UserIden::Uid,
                UserIden::Username,
                UserIden::Email,
                UserIden::SiteOwner,
            ])
            .from(UserIden::Table)
            .build_sqlx(PostgresQueryBuilder);

        let users = sqlx::query_as_with::<_, User, _>(&sql, values)
            .fetch_all(&self.db)
            .await?;

        Ok(users)
    }

    pub async fn get_votes(&self, zid: i32) -> Result<Vec<Vote>, PolisConnectionError> {
        let (sql, values) = Query::select()
            .columns([
                VoteIden::Pid,
                VoteIden::Zid,
                VoteIden::Tid,
                VoteIden::Vote,
                VoteIden::HighPriority,
            ])
            .from(VoteIden::Table)
            .and_where(Expr::col(VoteIden::Zid).eq(zid))
            .build_sqlx(PostgresQueryBuilder);

        println!("SQL: {}", sql);
        let votes = sqlx::query_as_with::<_, Vote, _>(&sql, values)
            .fetch_all(&self.db)
            .await?;

        Ok(votes)
    }

    pub async fn get_comments(&self, zid: i32) -> Result<Vec<Comment>, PolisConnectionError> {
        let (sql, values) = Query::select()
            .columns([
                CommentIden::Pid,
                CommentIden::Zid,
                CommentIden::Uid,
                CommentIden::Txt,
                CommentIden::Lang,
                CommentIden::Anon,
            ])
            .from(CommentIden::Table)
            .and_where(Expr::col(CommentIden::Zid).eq(zid))
            .build_sqlx(PostgresQueryBuilder);

        println!("SQL: {}", sql);
        let comments = sqlx::query_as_with::<_, Comment, _>(&sql, values)
            .fetch_all(&self.db)
            .await?;

        Ok(comments)
    }

    pub async fn get_math(&self, zid: i32) -> Result<Vec<Math>, PolisConnectionError> {
        let (sql, values) = Query::select()
            .columns([MathIden::Zid, MathIden::Data])
            .from(MathIden::Table)
            .and_where(Expr::col(MathIden::Zid).eq(zid))
            .build_sqlx(PostgresQueryBuilder);

        println!("SQL: {}", sql);
        let math = sqlx::query_as_with::<_, Math, _>(&sql, values)
            .fetch_all(&self.db)
            .await?;

        Ok(math)
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[tokio::test]
    async fn getting_votes() {
        let polis = PolisConnector::new(
            "postgres://postgres:polis@localhost:5431/polis-dev",
            "https://localhost:3000",
        )
        .await
        .unwrap();

        let users = polis.get_active_users(12).await.unwrap();
        println!("{users:#?}");
    }

    #[tokio::test]
    async fn getting_comments() {
        let polis = PolisConnector::new(
            "postgres://postgres:polis@localhost:5431/polis-dev",
            "https://localhost:3000",
        )
        .await
        .unwrap();

        let comments = polis.get_comments(12).await.unwrap();
        println!("{comments:#?}");
    }

    #[tokio::test]
    async fn test_deserialize_math() {
        let math_string =
            std::fs::read_to_string("/home/stuart/crown_shy/comhairle/adaptors/data_example.json")
                .unwrap();
        let math: MathData = serde_json::from_str(&math_string).unwrap();
        println!("{math:#?}");
    }

    #[tokio::test]
    async fn getting_math() {
        let polis = PolisConnector::new(
            "postgres://postgres:polis@localhost:5431/polis-dev",
            "https://localhost:3000",
        )
        .await
        .unwrap();

        let math = polis.get_math(12).await.unwrap();
        println!("{math:#?}");
    }
}
