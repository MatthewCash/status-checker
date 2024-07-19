use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serde::Serialize;
use service::{Service, ServiceData};

mod service;
mod services;

#[derive(Debug, Serialize)]
struct Section {
    time: u64,
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
    Ok(Section {
        time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        items: vec![
            SectionItem::Service(check!(homepage::HomepageCheck).await?),
            SectionItem::Service(check!(epsilon::EpsilonCheck).await?),
            SectionItem::SubSection(SubSection {
                name: "Minecraft".into(),
                desc: "Minecraft Infrastructure".into(),
                items: vec![SectionItem::Service(check!(panel::PanelCheck).await?)],
            }),
        ],
    })
}

#[tokio::main]
async fn main() {
    let global_status: Section = get_global_status().await.expect("Failed to get status!");
    println!("{}", serde_json::to_string_pretty(&global_status).unwrap());
}
