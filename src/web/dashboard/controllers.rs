use crate::models::_entities::users::Model;
use loco_rs::{controller::middleware::auth::JWTWithUser, prelude::*};

pub(super) async fn greet(
    auth: JWTWithUser<Model>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    super::views::dashboard(&v, &auth.user)
}
