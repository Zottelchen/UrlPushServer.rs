use actix_web::{middleware::Logger, web, App, HttpServer};
use pretty_env_logger;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use dotenv::dotenv;

mod tools;
mod urlserver;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Get address and say hello
    let address: &str = &env::var("PS_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let port: u16 = env::var("PS_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    pretty_env_logger::init_custom_env("PS_LOG_LEVEL");
    println!("Starting server at http://{}:{}", address, port);

    // Prepare Api Docs
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Url Push Server",
            version = "1.0.0",
            description = "Url Push Server",
            license(name = "WTFPL", url = "https://choosealicense.com/licenses/wtfpl/")
        ),
        paths(
            tools::hello,
            tools::echo,
            tools::ip,
            tools::query_echo,
            urlserver::get,
            urlserver::add
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    //Start server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/tools")
                    .route("/hello", web::get().to(tools::hello))
                    .route("/echo", web::post().to(tools::echo))
                    .route("/ip", web::get().to(tools::ip))
                    .route("/queryecho", web::get().to(tools::query_echo)),
            )
            .service(
                web::scope("/urls")
                    .route("/get", web::get().to(urlserver::get))
                    .route("/add", web::post().to(urlserver::add)),
            )
            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
    })
    .bind((address, port))?
    .run()
    .await
}
