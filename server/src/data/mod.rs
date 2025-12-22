pub mod config;

use crate::data::config::Config;
use std::sync::{Arc, RwLock};

pub struct AppState {
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Arc<RwLock<AppState>> {
        Arc::new(RwLock::new(AppState { config }))
    }
}
