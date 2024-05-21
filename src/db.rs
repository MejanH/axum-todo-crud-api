use std::sync::{Arc, RwLock};

use crate::models::{Todo, DB};

pub fn init_db() -> DB {
    Arc::new(RwLock::new(Vec::new()))
}
