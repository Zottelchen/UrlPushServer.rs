use actix_web::{web, App, HttpServer};
use utoipa::{
    openapi::schema::{},
    IntoParams, OpenApi, PartialSchema, ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;

mod tools;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get address and say hello
    let address: &str = "127.0.0.1";
    let port: u16 = 8080;
    println!("Starting server at http://{}:{}", address, port);

    // Prepare Api Docs
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Url Push Server",
            version = "1.0.0",
            description = "Url Push Server",
            contact(name = "Zottelchen", email = "urlpushserver@zottelchen.com"),
            license(name = "WTFPL", url = "https://choosealicense.com/licenses/wtfpl/")
        ),
        paths(tools::hello, tools::echo, tools::ip)
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    //Start server
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/tools")
                    .route("/hello", web::get().to(tools::hello))
                    .route("/echo", web::post().to(tools::echo))
                    .route("/ip", web::get().to(tools::ip)),
            )
            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
    })
    .bind((address, port))?
    .run()
    .await
}
