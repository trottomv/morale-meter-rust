use actix_web::{
    middleware,
    web,
    App,
    HttpServer
};
mod api;
mod index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index::root))
            .service(
                web::scope("/api/v1")
                .service(web::resource("").to(index::api_root))
                .service(web::resource("/surveys").to(api::surveys_list))
            )

    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
