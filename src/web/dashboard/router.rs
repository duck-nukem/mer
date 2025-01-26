use loco_rs::prelude::*;

use super::controllers::greet;

pub fn routes() -> Routes {
    Routes::new().add("/", get(greet))
}
