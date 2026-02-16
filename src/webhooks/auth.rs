use axum::{body::Body, extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::Response};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn extract_signature(headers: &HeaderMap) -> Result<&str, StatusCode> {
    let signature = headers
        .get("x-hub-signature-256")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(signature)
}

fn webhook_secret_as_hmac() -> Result<HmacSha256, StatusCode> {
    let secret = std::env::var("GITHUB_WEBHOOK_SECRET")
        .expect("GITHUB_WEBHOOK_SECRET must be set");

    let mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(mac)
}

pub async fn verify_github_signature(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = request.into_parts();

    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut mac = webhook_secret_as_hmac()?;
    mac.update(&body_bytes);

    let signature = extract_signature(&headers)?;
    let expected = format!("sha256={}", bytes_to_hex(&mac.finalize().into_bytes()));

    if signature != expected {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let new_request = Request::from_parts(
        parts,
        Body::from(body_bytes),
    );

    Ok(next.run(new_request).await)
}