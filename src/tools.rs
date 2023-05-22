use actix_web::{http::header::ContentType, HttpRequest, HttpResponse, Responder};

#[utoipa::path(get, path="/tools/hello",
    responses(
        (status = 200, description = "OK - should return 'Hello world!'", content_type="text/plain" ),
    ),
    tag = "Tools"
)]
pub async fn hello() -> impl Responder {
    "Hello world!"
}

#[utoipa::path(post, path="/tools/echo",
    responses(
        (status = 200, description = "OK - returns submitted text", content_type="text/plain"),
    ),
    tag = "Tools",
    request_body(content = String, description = "Any text content", content_type = "text/plain"),
)]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(req_body)
}

#[utoipa::path(get, path="/tools/ip",
    responses(
        (status = 200, description = "OK - returns IP of requester", content_type="text/plain"),
        (status = 422, description = "Unprocessable Entity - IP could not be determined", content_type="text/plain")
    ),
    tag = "Tools"
)]
pub async fn ip(req: HttpRequest) -> HttpResponse {
    // Retrieve the IP address from the request headers
    if let Some(addr) = req.connection_info().realip_remote_addr() {
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(addr.to_string());
    }

    // If the IP address couldn't be determined, return a default value
    return HttpResponse::UnprocessableEntity()
        .content_type(ContentType::plaintext())
        .body("Unknown".to_string());
}