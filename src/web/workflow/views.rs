use loco_rs::prelude::*;

use crate::models::users::Model;

pub(super) fn render<T: ViewRenderer>(
    v: &T,
    user_data: &Model,
    nonce: &str,
) -> Result<impl IntoResponse + use<T>> {
    format::render().view(
        v,
        "workflow/dist/index.html",
        data!({"user": user_data, "nonce": nonce}),
    )
}
