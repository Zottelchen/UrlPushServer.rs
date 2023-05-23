use actix_web::{http::header::ContentType, HttpRequest, HttpResponse, Responder};

#[utoipa::path(get, path="/get", context_path="/urls",
    responses(
        (status = OK, description = "OK - should return 'Hello world!'", content_type="text/plain" ),
    ),
    tag = "URL Server"
)]
pub async fn get() -> impl Responder {
    "Hello world!"
}

#[utoipa::path(post, path="/add", context_path="/urls",
    responses(
        (status = OK, description = "OK - returns submitted text", content_type="text/plain"),
    ),
    tag = "URL Server",
    request_body(content = String, description = "Any text content", content_type = "text/plain"),
)]
pub async fn add(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(req_body)
}

