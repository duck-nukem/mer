use crate::{models::_entities::users::Model, web::security::CspNonce};
use axum::{
    response::{sse::Event, Sse},
    Extension,
};
use futures::stream::{self, Stream};
use loco_rs::{controller::middleware::auth::JWTWithUser, prelude::*};
use std::{convert::Infallible, time::Duration};
use tokio::time::sleep;

pub(super) async fn greet(
    auth: JWTWithUser<Model>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(nonce): Extension<CspNonce>,
) -> Result<impl IntoResponse> {
    super::views::dashboard(&v, &auth.user, nonce.value.as_str())
}

pub(super) async fn keep_updating(
    _auth: JWTWithUser<Model>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::unfold(0, |count| async move {
        sleep(Duration::from_secs(2)).await;
        let event = Event::default().data(format!("Message #{count}"));
        Some((Ok(event), count + 1))
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
