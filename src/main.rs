use anyhow::{bail, Result};
use futures::future::try_join_all;
use itertools::Itertools;
use serde::Serialize;
use service::{Service, ServiceData, State};
use std::time::{SystemTime, UNIX_EPOCH};

mod service;
mod services;

#[derive(Debug, Serialize)]
struct Section {
    time: u64,
    overall_state: State,
    items: Vec<SectionItem>,
}

#[derive(Debug, Serialize)]
struct SubSection {
    name: String,
    desc: String,
    items: Vec<SectionItem>,
}

#[derive(Debug, Serialize)]
enum SectionItem {
    Service(ServiceData),
    SubSection(SubSection),
}

macro_rules! check {
    ( $start:ident$(::$path_seg:ident)* ) => {
        (services::$start$(::$path_seg)* {}).get_data()
    };
}

async fn get_global_status() -> Result<Section> {
    let mut checks = try_join_all([
        check!(homepage::HomepageCheck),
        check!(epsilon::EpsilonCheck),
        check!(panel::PanelCheck),
        check!(minecraft_java::MinecraftJavaCheck),
        check!(minecraft_bedrock::MinecraftBedrockCheck),
    ])
    .await?;

    let overall_state = if checks.iter().all(|x| x.status.state == State::Healthy) {
        State::Healthy
    } else if checks.iter().all(|x| x.status.state == State::Offline) {
        State::Offline
    } else {
        State::Unhealthy
    };

    let Some((homepage, epsilon, panel, minecraft_java, minecraft_bedrock)) =
        checks.drain(..).tuples().next()
    else {
        bail!("Some checks are missing!")
    };

    Ok(Section {
        time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        overall_state,
        items: vec![
            SectionItem::Service(homepage),
            SectionItem::Service(epsilon),
            SectionItem::SubSection(SubSection {
                name: "Minecraft".into(),
                desc: "Minecraft Infrastructure".into(),
                items: vec![
                    SectionItem::Service(panel),
                    SectionItem::Service(minecraft_java),
                    SectionItem::Service(minecraft_bedrock),
                ],
            }),
        ],
    })
}

#[tokio::main]
async fn main() {
    let global_status: Section = get_global_status().await.expect("Failed to get status!");
    println!("{}", serde_json::to_string_pretty(&global_status).unwrap());
}
