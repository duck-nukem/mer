use loco_rs::prelude::Routes;

use crate::web::{authn::middlewares::redirect_to_login_on_unauthorized, security::set_csp_header};

/// Add your module routes here; common middlewares will be applied. See `with_middlewares`
#[must_use]
pub fn with_routes() -> Vec<Routes> {
    vec![
        crate::web::dashboard::router::routes(),
        crate::web::workflow::router::routes(),
    ]
}

/// Define the middlewares you'd want to apply by default to `with_routes`
pub fn with_middlewares(route: Routes) -> Routes {
    route
        .layer(axum::middleware::from_fn(set_csp_header))
        .layer(axum::middleware::from_fn(redirect_to_login_on_unauthorized))
}
