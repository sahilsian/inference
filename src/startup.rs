use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpServer, middleware, web};
use sqlx::PgPool;
use crate::routes::events::fire::{
    get_fire_history,
    get_live_feed,
    get_incident,
    refresh_fires
};
use crate::routes::diagnostics::{
    health_check
};

pub fn startup(listener: TcpListener, database_pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(database_pool);

    let server = HttpServer::new(move || {
        App::new()
            // Logging Middlewear
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                .service(
                    web::scope("/events")
                    .service(
                        // Nasa FIRMS Fire Event Ingest
                        web::scope("/fire")
                        .route("/live", web::get().to(get_live_feed))
                        .route("/history", web::get().to(get_fire_history))
                        .route("/refresh", web::post().to(refresh_fires))
                        .route("/incident/{id}", web::get().to(get_incident))
                    )
                )
                .service(
                    web::scope("/diagnostics")
                    .route("health_check", web::get().to(health_check))
                )
            )
            .app_data(pool.clone()) 
        })
        .listen(listener)?
        .run();

    Ok(server)
}