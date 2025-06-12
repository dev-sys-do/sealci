use std::{fmt::Display, sync::Arc};

use tokio::sync::RwLock;

pub struct SealedService<App, Config>
where
    App: sealcid_traits::App<Config>,
    Config: Display,
{
    pub app: Arc<RwLock<App>>,
    enabled: Arc<RwLock<bool>>,
    pub config: Arc<RwLock<Config>>,
}

impl<App, Config> SealedService<App, Config>
where
    App: sealcid_traits::App<Config>,
    Config: Display + Clone,
{
    pub async fn restart_with_config(
        &self,
        config: impl Into<Config> + Clone,
    ) -> Result<(), App::Error> {
        if let Err(_) = self.app.read().await.stop().await {
            tracing::error!("Failed to stop the app {}", self.app.read().await.name());
        }
        *self.app.write().await = App::configure(config.clone().into()).await?;
        *self.config.write().await = config.clone().into();
        let enabled = *self.enabled.read().await;
        if enabled {
            self.app.read().await.run().await?;
        }
        Ok(())
    }

    pub async fn restart(&self) -> Result<(), App::Error> {
        let enabled = *self.enabled.read().await;
        if enabled {
            let app = self.app.read().await;
            if let Err(_) = app.stop().await {
                tracing::error!("Failed to stop the app {}", app.name());
            }
            if let Err(_) = app.run().await {
                tracing::error!("Failed to start the app {}", app.name());
            }
        } else {
            tracing::warn!(
                "App {} is not enabled, skipping restart",
                self.app.read().await.name()
            );
        }
        Ok(())
    }

    pub fn new(app: App, config: impl Into<Config>) -> Self {
        Self {
            app: Arc::new(RwLock::new(app)),
            enabled: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(config.into())),
        }
    }

    pub async fn is_enabled(&self) -> bool {
        *self.enabled.read().await
    }

    pub async fn enable(&self) -> Result<(), App::Error> {
        *self.enabled.write().await = true;
        Ok(())
    }

    pub async fn disable(&self) -> Result<(), App::Error> {
        *self.enabled.write().await = false;
        Ok(())
    }
}
