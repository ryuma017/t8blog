use t8blog::{configuration::get_configuration, startup::Application, telemetry::{get_subscriber, init_subscriber}};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("t8blog".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_server().await?;
    Ok(())
}
