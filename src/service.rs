use anyhow::Result;
use async_trait::async_trait;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct ServiceInfo {
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Serialize)]
pub struct ServiceData {
    pub name: String,
    pub desc: String,
    pub status: Status,
}

#[derive(Debug, Serialize)]
pub enum State {
    Healthy,
    Unhealthy,
    Offline,
}

#[derive(Debug, Serialize)]
pub struct Status {
    pub state: State,
    pub text: String,
}

#[async_trait]
pub trait Service {
    fn service_info(&self) -> ServiceInfo;
    async fn get_status(&self) -> Result<Status>;
    async fn get_data(&self) -> Result<ServiceData> {
        Ok(ServiceData {
            name: self.service_info().name,
            desc: self.service_info().desc,
            status: self.get_status().await?,
        })
    }
}
