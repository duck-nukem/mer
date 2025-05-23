use crate::{
    mailers::auth::AuthMailer,
    models::{
        _entities::users,
        users::{LoginParams, RegisterParams},
    },
    web::authn::views::CurrentResponse,
};
use axum::{
    Form,
    body::Body,
    debug_handler,
    extract::Query,
    http::{
        HeaderValue, StatusCode,
        header::{LOCATION, SET_COOKIE},
    },
    response::Redirect,
};
use cookie::{Cookie, SameSite};
use loco_rs::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use super::views::{LoginPageState, LoginPageStatus, ResetPasswordPageState};

pub static EMAIL_DOMAIN_RE: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MagicLinkParams {
    pub email: String,
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
pub(super) async fn register(
    State(ctx): State<AppContext>,
    Form(params): Form<RegisterParams>,
) -> Result<Response> {
    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            return Ok(Redirect::to("/api/auth/register?status=error").into_response());
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    Ok(Redirect::to("/api/auth/login").into_response())
}

#[debug_handler]
pub(super) async fn verify_email_via_token(
    State(ctx): State<AppContext>,
    Path(token): Path<String>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_verification_token(&ctx.db, &token).await else {
        return Ok(
            Redirect::to("/api/auth/login?status=error&message=invalid_token").into_response(),
        );
    };

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }

    Ok(Redirect::to("/api/auth/login?status=success&message=verification_success").into_response())
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
pub(super) async fn forgot(
    State(ctx): State<AppContext>,
    Form(params): Form<ForgotParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return format::json(());
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;
    Ok(Redirect::to("/api/auth/mail-sent").into_response())
}

pub(super) async fn render_reset_password_form(
    ViewEngine(v): ViewEngine<TeraView>,
    Query(query_params): Query<ResetPasswordPageState>,
) -> Result<impl IntoResponse> {
    super::views::render_reset_password_form(&v, &query_params)
}

/// reset user password by the given parameters
#[debug_handler]
pub(super) async fn reset(
    State(ctx): State<AppContext>,
    Form(params): Form<ResetParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    Ok(Redirect::to("/api/auth/login?status=success&message=password_changed").into_response())
}

#[allow(clippy::unused_async)]
pub(super) async fn render_login_form(
    ViewEngine(v): ViewEngine<TeraView>,
    Query(query_params): Query<LoginPageState>,
    form: Option<Form<LoginParams>>,
) -> Result<impl IntoResponse> {
    super::views::login_form(&v, &query_params, form.as_ref())
}

#[allow(clippy::unused_async)]
pub(super) async fn render_default_login_form(
    ViewEngine(v): ViewEngine<TeraView>,
    Query(query_params): Query<LoginPageState>,
) -> Result<impl IntoResponse> {
    let form = Form(LoginParams {
        email: String::new(),
        password: String::new(),
    });
    render_login_form(ViewEngine(v), Query(query_params), Some(form)).await
}

pub(super) async fn render_forgotten_password_form(
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    super::views::forgotten_password_form(&v)
}

pub(super) async fn mail_sent_acknowledgement(
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    super::views::mail_sent_acknowledgement(&v)
}

#[allow(clippy::unwrap_used, clippy::unused_async)]
pub(super) async fn logout() -> Result<Response> {
    let response_builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(
            SET_COOKIE,
            "jwt=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT",
        )
        .header(LOCATION, "/api/auth/login")
        .body(Body::empty());

    Ok(response_builder.unwrap())
}

#[allow(clippy::expect_used)]
fn generate_login_response(token: &str) -> Response<Body> {
    let auth_cookie = Cookie::build(("jwt", token))
        .path("/")
        .same_site(SameSite::Strict)
        .http_only(true);
    let mut response = Redirect::to("/").into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(auth_cookie.to_string().as_str())
            .expect("Can't construct auth cookie header, crashing!"),
    );

    response
}

#[debug_handler]
pub(super) async fn login(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Form(params): Form<LoginParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_verified_email(&ctx.db, &params.email).await else {
        return Ok(render_login_form(
            ViewEngine(v),
            Query(LoginPageState {
                status: Some(LoginPageStatus::ERROR),
                message: Some(String::from("invalid_credentials")),
            }),
            Some(Form(params)),
        )
        .await
        .into_response());
    };

    let is_password_correct = user.verify_password(&params.password);
    if !is_password_correct {
        return Ok(render_login_form(
            ViewEngine(v),
            Query(LoginPageState {
                status: Some(LoginPageStatus::ERROR),
                message: Some(String::from("invalid_credentials")),
            }),
            Some(Form(params)),
        )
        .await
        .into_response());
    }

    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    Ok(generate_login_response(token.as_str()).into_response())
}

#[debug_handler]
pub(super) async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

/// Magic link authentication provides a secure and passwordless way to log in to the application.
///
/// # Flow
/// 1. **Request a Magic Link**:
///    A registered user sends a POST request to `/magic-link` with their email.
///    If the email exists, a short-lived, one-time-use token is generated and sent to the user's email.
///    For security and to avoid exposing whether an email exists, the response always returns 200, even if the email is invalid.
///
/// 2. **Click the Magic Link**:
///    The user clicks the link (/magic-link/{token}), which validates the token and its expiration.
///    If valid, the server generates a JWT and responds with a [`LoginResponse`].
///    If invalid or expired, an unauthorized response is returned.
///
/// This flow enhances security by avoiding traditional passwords and providing a seamless login experience.
pub(super) async fn magic_link(
    State(ctx): State<AppContext>,
    Form(params): Form<MagicLinkParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // return success to the caller
        tracing::debug!(email = params.email, "user not found by email");
        return format::empty_json();
    };

    let user = user.into_active_model().create_magic_link(&ctx.db).await?;
    AuthMailer::send_magic_link(&ctx, &user).await?;

    Ok(Redirect::to("/api/auth/mail-sent").into_response())
}

/// Verifies a magic link token and authenticates the user.
pub(super) async fn magic_link_verify(
    Path(token): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_magic_token(&ctx.db, &token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // return success to the caller - WTF? How is unauthorized a success? TODO: Improve this
        return unauthorized("unauthorized!");
    };

    let user = user.into_active_model().clear_magic_link(&ctx.db).await?;

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    Ok(generate_login_response(token.as_str()).into_response())
}

pub(super) async fn render_signup_form(
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    super::views::signup_form(&v)
}
