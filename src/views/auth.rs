use crate::models::{_entities::users, users::LoginParams};
use axum::response::IntoResponse;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

pub fn login_form(
    v: &impl ViewRenderer,
    form: Option<&Form<LoginParams>>,
) -> Result<impl IntoResponse> {
    let template_data = form.as_ref().map_or_else(
        || LoginParams {
            email: String::new(),
            password: String::new(),
        },
        |original| LoginParams {
            email: original.email.clone(),
            password: original.password.clone(),
        },
    );

    format::render().view(v, "auth/login.html", template_data)
}

pub fn signup_form(v: &impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(v, "auth/signup.html", data!({}))
}
