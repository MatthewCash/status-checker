use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;

use crate::service::{Service, ServiceInfo, State, Status};

pub struct MinecraftBedrockCheck {}

#[async_trait]
impl Service for MinecraftBedrockCheck {
    fn service_info(&self) -> ServiceInfo {
        ServiceInfo {
            name: "Minecraft Bedrock".into(),
            desc: "Bedrock Proxy for Minecraft Servers".into(),
        }
    }
    async fn get_status(&self) -> Result<Status> {
        Ok(
            match elytra_ping::bedrock::ping(
                ("matthew-cash.com".into(), 19132),
                Duration::from_secs(5),
                1,
            )
            .await
            {
                Ok((info, _latency)) => Status {
                    state: State::Healthy,
                    text: format!("Server is online with version {}", info.mc_version),
                },
                Err(why) => Status {
                    state: State::Offline,
                    text: format!("Ping failed: {why}"),
                },
            },
        )
    }
}
