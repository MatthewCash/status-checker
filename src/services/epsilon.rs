use anyhow::Result;
use async_trait::async_trait;

use crate::service::{Service, ServiceInfo, State, Status};

pub struct EpsilonCheck {}

#[async_trait]
impl Service for EpsilonCheck {
    fn service_info(&self) -> ServiceInfo {
        ServiceInfo {
            name: "Epsilon Node".into(),
            desc: "Main Compute Server".into(),
        }
    }
    async fn get_status(&self) -> Result<Status> {
        let res = reqwest::get("https://epsilonstatus.matthew-cash.com/check").await?;

        let code = res.status().as_u16();

        Ok(if code == 200 {
            Status {
                state: State::Healthy,
                text: "Machine is reachable".into(),
            }
        } else {
            Status {
                state: State::Offline,
                text: res.text().await?,
            }
        })
    }
}
