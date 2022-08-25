use std::{net::TcpListener, time::Duration};

use actix_web::{
    dev::Server,
    middleware::NormalizePath,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_actix_web::TracingLogger;

use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{create_user, health_check};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = format!(
            "{}:{}",
            configuration.application.host(),
            configuration.application.port()
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = build_server(listener, connection_pool)?;

        Ok(Self { port, server })
    }

    pub async fn run_server(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub fn get_connection_pool(database_settings: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy_with(database_settings.connect_options_with_db())
}

fn build_server(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection_pool = Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(NormalizePath::trim())
            .route("/health_check", web::get().to(health_check))
            .service(
                web::scope("/users")
                    .route("", web::post().to(create_user))
                    .service(
                        web::resource("/{name}")
                            .name("user_detail")
                            .route(web::get().to(actix_web::HttpResponse::Ok)) // TODO
                            .route(web::patch().to(actix_web::HttpResponse::Ok)) // TODO
                            .route(web::delete().to(actix_web::HttpResponse::Ok)) // TODO
            )
            .app_data(connection_pool.clone())
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
