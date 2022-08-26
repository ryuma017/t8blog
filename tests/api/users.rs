use serde_json::Value;

use crate::helpers::spawn_app;

#[tokio::test]
async fn post_users_persists_the_new_user() {
    // Arrange
    let app = spawn_app().await;
    let json = serde_json::json!({
        "name": "username"
    });

    // Act
    app.post_users(&json).await.json::<Value>().await.unwrap();

    // Assert
    let saved = sqlx::query!("SELECT name FROM users;")
        .fetch_one(&app.connection_pool)
        .await
        .expect("Failed to fetch a saved user");

    assert_eq!("username", saved.name);
}

#[tokio::test]
async fn get_users_returns_the_user_info() {
    // Arrange
    let app = spawn_app().await;

    let json = serde_json::json!({
        "name": "user1"
    });
    // create a user
    let post_result_json = app.post_users(&json).await.json::<Value>().await.unwrap();

    // Act
    let get_result_json = app
        .get_users(post_result_json["data"]["id"].as_i64().unwrap())
        .await
        .json::<Value>()
        .await
        .unwrap();

    // Assert
    assert_eq!(post_result_json, get_result_json);
}
