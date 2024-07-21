use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;

use crate::service::{Service, ServiceInfo, State, Status};

pub struct MinecraftJavaCheck {}

#[async_trait]
impl Service for MinecraftJavaCheck {
    fn service_info(&self) -> ServiceInfo {
        ServiceInfo {
            name: "Minecraft Java".into(),
            desc: "Java Proxy for Minecraft Servers".into(),
        }
    }
    async fn get_status(&self) -> Result<Status> {
        Ok(
            match elytra_ping::ping_or_timeout(
                ("matthew-cash.com".to_string(), 25565),
                Duration::from_secs(5),
            )
            .await
            {
                Ok((info, _latency)) => Status {
                    state: State::Healthy,
                    text: format!(
                        "Server is online with version {}",
                        info.version
                            .map(|ver| ver.name)
                            .unwrap_or("Unknown Version".into())
                    ),
                },
                Err(why) => Status {
                    state: State::Offline,
                    text: format!("Ping failed: {why}"),
                },
            },
        )
    }
}
