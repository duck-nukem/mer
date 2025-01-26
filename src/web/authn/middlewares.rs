use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{Redirect, Response},
};
use loco_rs::prelude::*;

pub async fn redirect_to_login_on_unauthorized(req: Request, next: Next) -> Response {
    let response = next.run(req).await;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Redirect::to("/api/auth/login").into_response();
    }

    response
}
