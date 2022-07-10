use t8blog::{
    configuration::get_configuration,
    startup::Application,
    telemetry::init_telemetry,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_telemetry();

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_server().await?;
    Ok(())
}
