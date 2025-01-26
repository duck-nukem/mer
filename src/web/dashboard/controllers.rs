use crate::{models::_entities::users::Model, web::security::CspNonce};
use axum::Extension;
use loco_rs::{controller::middleware::auth::JWTWithUser, prelude::*};

pub(super) async fn greet(
    auth: JWTWithUser<Model>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(_nonce): Extension<CspNonce>,
) -> Result<impl IntoResponse> {
    super::views::dashboard(&v, &auth.user)
}
