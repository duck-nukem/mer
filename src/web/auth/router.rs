use loco_rs::prelude::*;

use super::controllers::*;

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth")
        .add("/register", post(register))
        .add("/register", get(render_signup_form))
        .add("/verify/{token}", get(verify))
        .add("/login", get(|v| render_login_form(v, None)))
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/current", get(current))
        .add("/magic-link", post(magic_link))
        .add("/magic-link/{token}", get(magic_link_verify))
}
