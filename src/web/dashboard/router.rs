use loco_rs::prelude::*;

use crate::web::authn::middlewares::redirect_to_login_on_unauthorized;

use super::controllers::greet;

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(greet))
        .layer(axum::middleware::from_fn(redirect_to_login_on_unauthorized))
}
