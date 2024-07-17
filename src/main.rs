use anyhow::Result;
use serde::Serialize;
use service::{Service, ServiceData};

mod service;
mod services;

#[derive(Debug, Serialize)]
struct Section {
    name: String,
    desc: String,
    service: Box<SectionService>,
}

#[derive(Debug, Serialize)]
enum SectionService {
    Service(ServiceData),
    Services(Vec<SectionService>),
    Section(Section),
}

macro_rules! check {
    ( $start:ident$(::$path_seg:ident)* ) => {
        (services::$start$(::$path_seg)* {}).get_data()
    };
}

async fn get_global_status() -> Result<Section> {
    Ok(Section {
        name: "Global".into(),
        desc: "Global Desciption".into(),
        service: Box::new(SectionService::Services(vec![
            SectionService::Service(check!(homepage::HomepageCheck).await?),
            SectionService::Service(check!(epsilon::EpsilonCheck).await?),
        ])),
    })
}

#[tokio::main]
async fn main() {
    let global_status: Section = get_global_status().await.expect("Failed to get status!");
    println!("{}", serde_json::to_string_pretty(&global_status).unwrap());
}
