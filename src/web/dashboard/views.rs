use loco_rs::prelude::*;

use crate::models::users::Model;

pub(super) fn dashboard(
    v: &impl ViewRenderer,
    user_data: &Model,
    nonce: &str,
) -> Result<impl IntoResponse> {
    format::render().view(v, "home.html", data!({"user": user_data, "nonce": nonce}))
}
