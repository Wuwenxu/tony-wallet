use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;


use crate::{config, routes};
use crate::bootstrap::result::Response;


pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let server=HttpServer::new(move || {
        let data = web::Data::new(super::database::connection());

        App::new()
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::clone(&data))
            .service(
                web::scope("/api")
                    .configure(routes::admin::register_routes),
            )
            .route("/test", web::get().to(|| async { "Hello, World!" }))
    })
    .bind((config::APP.host.as_str(), config::APP.port))?
    .run() ;
    println!("Server running at http://{}/", config::APP.host.as_str());

    server.await
}
