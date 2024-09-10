use interactor::ContractInteract;
use std::sync::{Arc, RwLock};

pub struct AppState {
    pub interactor: Arc<RwLock<ContractInteract>>,
}
