use axum::{
    extract::Request,
    http::{header::CONTENT_SECURITY_POLICY, HeaderValue},
    middleware::Next,
    response::Response,
};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct CspNonce {
    pub value: String,
}

#[allow(clippy::expect_used)]
async fn generate_async_safe_nonce() -> String {
    tokio::task::spawn_blocking(|| {
        let mut rng = rand::thread_rng();
        rng.gen_range(10_000_000..100_000_000).to_string()
    })
    .await
    .expect("Failed to generate nonce, can't set CSP nonce")
}

/// Generates a nonce and sets it as the CSP Header and also
/// exposes it as an extension
///
/// To use it in a view add `Extension(nonce): Extension<CspNonce>` to
/// its arguments, and then pass `nonce` to the template.
///
/// In HTML `<script nonce={{ nonce }}>...</script>` should be sufficient
/// to execute scripts.
///
/// # Panics
/// This middleware is afraid of everything, if it can't read, construct, or set
/// any of the relevant data (headers, nonce), it will crash the application.
///
/// The reasoning is it that it should be simple enough to always work,
/// and if it doesn't it's better to not run at all, then to run insecurely.
#[allow(clippy::expect_used)]
pub async fn set_csp_header(mut request: Request, next: Next) -> Response {
    let nonce = generate_async_safe_nonce().await;
    request.extensions_mut().insert(CspNonce {
        value: nonce.clone(),
    });

    let header_value = format!("object-src 'none'; base-uri 'none'; script-src 'nonce-{nonce}'");
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_bytes(header_value.as_bytes())
            .expect("Unable to construct security headers"),
    );

    response
}
