use crate::models::{_entities::users, users::LoginParams};
use axum::response::IntoResponse;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
pub(super) struct CurrentResponse {
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LoginPageStatus {
    SUCCESS,
    ERROR,
}

impl LoginPageStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match &self {
            Self::SUCCESS => "success",
            Self::ERROR => "error",
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPageState {
    pub status: Option<LoginPageStatus>,
    pub message: Option<String>,
}

pub(super) fn login_form<T: ViewRenderer>(
    v: &T,
    page_state: &LoginPageState,
    form: Option<&Form<LoginParams>>,
) -> Result<impl IntoResponse + use<T>> {
    let form_state = form.as_ref().map_or_else(
        || LoginParams {
            email: String::new(),
            password: String::new(),
        },
        |original| LoginParams {
            email: original.email.clone(),
            password: original.password.clone(),
        },
    );

    format::render().view(
        v,
        "auth/login.html",
        data!({"form": form_state, "page_state": page_state}),
    )
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetPasswordPageState {
    pub token: String,
}

pub(super) fn render_reset_password_form<T: ViewRenderer>(
    v: &T,
    query_params: &ResetPasswordPageState,
) -> Result<impl IntoResponse + use<T>> {
    format::render().view(v, "auth/reset.html", data!(query_params))
}

pub(super) fn signup_form<T: ViewRenderer>(v: &T) -> Result<impl IntoResponse + use<T>> {
    format::render().view(v, "auth/signup.html", data!({}))
}

pub(super) fn forgotten_password_form<T: ViewRenderer>(
    v: &T,
) -> Result<impl IntoResponse + use<T>> {
    format::render().view(v, "auth/forgot_password.html", data!({}))
}

pub(super) fn mail_sent_acknowledgement<T: ViewRenderer>(
    v: &T,
) -> Result<impl IntoResponse + use<T>> {
    format::render().view(v, "auth/check_mail.html", data!({}))
}
