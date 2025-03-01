use axum::{
    extract::Request,
    http::{HeaderValue, header::CONTENT_SECURITY_POLICY},
    middleware::Next,
    response::Response,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct CspNonce {
    pub value: String,
}

/// Generate 16 random bytes (128 bits) and encodes it as Base64
/// See <https://www.w3.org/TR/CSP3/#security-nonces>
fn generate_nonce() -> String {
    let mut rng = rand::rng();
    let nonce_bytes: [u8; 16] = rng.random();

    STANDARD.encode(nonce_bytes)
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
    let nonce = generate_nonce();
    request.extensions_mut().insert(CspNonce {
        value: nonce.clone(),
    });

    let header_value = format!("base-uri 'self'; object-src 'none'; script-src 'nonce-{nonce}'");
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_bytes(header_value.as_bytes())
            .expect("Unable to construct security headers"),
    );

    response
}
