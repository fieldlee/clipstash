use crate::data::DatabasePool;
use crate::services;
use std::time::Duration;
use tokio::runtime::Handle;

pub struct Maintenance;

impl Maintenance {
    pub fn spawn(pool: DatabasePool, handle: Handle) -> Self {
        handle.spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Err(e) = services::action::delete_expired(&pool).await {
                    eprintln!("failed to delete expired clip: {}", e);
                }
            }
        });
        Self
    } 
}