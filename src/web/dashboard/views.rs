use loco_rs::prelude::*;

use crate::models::users::Model;

pub(super) fn dashboard<T: ViewRenderer>(
    v: &T,
    user_data: &Model,
    nonce: &str,
) -> Result<impl IntoResponse + use<T>> {
    format::render().view(v, "home.html", data!({"user": user_data, "nonce": nonce}))
}
