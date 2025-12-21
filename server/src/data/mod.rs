use std::sync::{Arc, RwLock};

pub struct AppState {
    pub min_broadcast_fee: u128,
}

impl AppState {
    pub fn new() -> Arc<RwLock<AppState>> {
        Arc::new(RwLock::new(AppState {
            min_broadcast_fee: 1,
        }))
    }
}
