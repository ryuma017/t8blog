use serde_json::Value;

use crate::helpers::spawn_app;

#[tokio::test]
async fn post_users_persists_the_new_user() {
    // Arrange
    let app = spawn_app().await;
    let username = "username";
    let json = serde_json::json!({
        "name": username
    });

    // Act
    let response = app.post_users(&json).await;

    // Assert
    assert_eq!(201, response.status().as_u16());

    let saved = sqlx::query!("SELECT name FROM users;")
        .fetch_one(&app.connection_pool)
        .await
        .expect("Failed to fetch a saved user");
    assert_eq!(username, saved.name);
}

#[tokio::test]
async fn get_users_returns_the_user_info() {
    // Arrange
    let app = spawn_app().await;

    let json = serde_json::json!({
        "name": "user1"
    });
    // create a user
    let response = app.post_users(&json).await;
    assert_eq!(201, response.status().as_u16());
    let post_result_json = response.json::<Value>().await.unwrap();

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

#[tokio::test]
async fn patch_users_changes_the_username() {
    // Arrange
    let app = spawn_app().await;
    let json = serde_json::json!({
        "name": "user1"
    });
    // create a user
    let response = app.post_users(&json).await;
    assert_eq!(201, response.status().as_u16());
    let post_result_json = response.json::<Value>().await.unwrap();
    let user_id = post_result_json["data"]["id"].as_i64().unwrap();

    let new_username = "new_username";
    let patch_json = serde_json::json!({
        "name": new_username
    });

    // Act
    let patch_result_json = app
        .patch_users(user_id, &patch_json)
        .await
        .json::<Value>()
        .await
        .unwrap();

    // Assert
    assert_eq!(Some("Ok"), patch_result_json["status"].as_str());

    let saved = sqlx::query!("SELECT name FROM users;")
        .fetch_one(&app.connection_pool)
        .await
        .expect("Failed to fetch a saved user");
    assert_eq!(new_username, saved.name);
}

#[tokio::test]
async fn delete_users_deletes_the_user() {
    // Arrange
    let app = spawn_app().await;
    let json = serde_json::json!({
        "name": "username"
    });
    // create a user
    let response = app.post_users(&json).await;
    assert_eq!(201, response.status().as_u16());
    let post_result_json = response.json::<Value>().await.unwrap();
    let user_id = post_result_json["data"]["id"].as_i64().unwrap();

    // Act
    let delete_result_json = app
        .delete_users(user_id)
        .await
        .json::<Value>()
        .await
        .unwrap();

    // Assert
    assert_eq!(Some("Ok"), delete_result_json["status"].as_str());

    let saved = sqlx::query!("SELECT name FROM users;")
        .fetch_optional(&app.connection_pool)
        .await
        .expect("Failed to fetch optional");
    assert!(saved.is_none());
}
