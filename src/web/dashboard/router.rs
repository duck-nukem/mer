use loco_rs::prelude::*;

use super::controllers::{greet, keep_updating};

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(greet))
        .add("/events", get(keep_updating))
}
