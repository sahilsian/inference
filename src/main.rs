use std::net::TcpListener;

use inference::startup::startup;
use inference::configuration::get_config;
use sqlx::PgPool;


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // main fn
    let configuration = get_config()
    .expect("Could not get configuration");

    let database_pool = PgPool::connect_lazy_with(configuration.database.connection_options());

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address).expect("Could not bind address to tcp listener");

    startup(listener, database_pool)?.await
}
