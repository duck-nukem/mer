use loco_rs::prelude::*;

use super::controllers::{
    current, forgot, login, logout, magic_link, magic_link_verify, register,
    render_default_login_form, render_signup_form, reset, verify_email_via_token,
};

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth")
        .add("/register", post(register))
        .add("/register", get(render_signup_form))
        .add("/verify/{token}", get(verify_email_via_token))
        .add("/login", get(render_default_login_form))
        .add("/login", post(login))
        .add("/logout", post(logout))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/current", get(current))
        .add("/magic-link", post(magic_link))
        .add("/magic-link/{token}", get(magic_link_verify))
}
