use anyhow::Result;
use async_trait::async_trait;

use crate::service::{Service, ServiceInfo, State, Status};

pub struct PanelCheck {}

#[async_trait]
impl Service for PanelCheck {
    fn service_info(&self) -> ServiceInfo {
        ServiceInfo {
            name: "Panel".into(),
            desc: "Web panel for game server management".into(),
        }
    }
    async fn get_status(&self) -> Result<Status> {
        let res = reqwest::get("https://panel.matthew-cash.com").await?;

        let code = res.status().as_u16();
        let body = res.text().await?;

        Ok(if code != 200 {
            Status {
                state: State::Offline,
                text: format!("Response is HTTP code {code}"),
            }
        } else if !body.starts_with("<!DOCTYPE html>") {
            Status {
                state: State::Unhealthy,
                text: "Response is not valid HTML".into(),
            }
        } else {
            Status {
                state: State::Healthy,
                text: "Service is online".into(),
            }
        })
    }
}
