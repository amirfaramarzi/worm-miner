pub mod config;

use crate::data::config::Config;
use alloy::{
    consensus::Header,
    providers::{Provider, RootProvider, fillers::FillProvider},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct AppStateGeneric<P: Provider> {
    pub config: Config,
    pub provider: P,
    pub header_cache: HashMap<u64, Header>,
}

pub type AppStateProvider = RootProvider;
pub type AppState = AppStateGeneric<AppStateProvider>;

impl AppState {
    pub fn new(config: Config, provider: AppStateProvider) -> Arc<RwLock<AppState>> {
        Arc::new(RwLock::new(AppState {
            config,
            provider,
            header_cache: Default::default(),
        }))
    }
}
