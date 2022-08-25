use crate::helpers::spawn_app;

#[tokio::test]
async fn post_users_persists_the_new_user() {
    // Arrange
    let app = spawn_app().await;
    let json = serde_json::json!({
        "name": "username"
    });

    // Act
    app.post_users(&json).await;

    // Assert
    let saved = sqlx::query!("SELECT name FROM users;").fetch_one(&app.connection_pool).await.expect("Failed to fetch a saved user");

    assert_eq!("username", saved.name);
}
