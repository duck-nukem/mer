use crate::{models::_entities::users::Model, web::security::CspNonce};
use axum::Extension;
use loco_rs::{controller::middleware::auth::JWTWithUser, prelude::*};

pub(super) async fn render(
    auth: JWTWithUser<Model>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(nonce): Extension<CspNonce>,
) -> Result<impl IntoResponse> {
    super::views::render(&v, &auth.user, nonce.value.as_str())
}
