use t8blog::{
    startup::Application,
    configuration::get_configuration
};

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.application.assign_random_port();
        c
    };

    let application = Application::build(configuration).await.expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_server());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
    }
}
