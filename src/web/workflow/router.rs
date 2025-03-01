use loco_rs::prelude::*;

use super::controllers::render;

pub fn routes() -> Routes {
    Routes::new().prefix("/workflow").add("/", get(render))
}
