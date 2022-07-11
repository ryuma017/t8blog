use once_cell::sync::Lazy;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use t8blog::{
    configuration::{get_configuration, DatabaseSettings},
    startup::{get_connection_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_owned();
    let subscriber_name = "test".to_owned();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub connection_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // テスト環境を分離するためにrandomise
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.assign_random_port();
        c
    };

    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_server());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        connection_pool: get_connection_pool(&configuration.database),
    }
}

async fn configure_database(database_settings: &DatabaseSettings) {
    // Create
    let mut connection =
        PgConnection::connect_with(&database_settings.connect_options_without_db())
            .await
            .expect("Failed to connect to Postgres.");
    connection
        .execute(&*format!(
            r#"CREATE DATABASE "{}";"#,
            database_settings.database_name
        ))
        .await
        .expect("Failed to create database.");

    // Migrate
    let connection_pool = PgPool::connect_with(database_settings.connect_options_with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
}
