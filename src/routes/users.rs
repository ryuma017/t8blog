use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// TODO: Error handling (return `Result<HttpResponse, Error>`)

#[derive(Deserialize, Serialize)]
struct Users {
    id: i64,
    name: String,
    create_timestamp: NaiveDateTime,
    update_timestamp: NaiveDateTime,
}

#[derive(serde::Deserialize)]
pub struct Body {
    name: String,
}

// `POST /users`
pub async fn post_users(json: web::Json<Body>, pool: web::Data<PgPool>) -> HttpResponse {
    let username = &*json.name;

    let user = insert_user(username, &pool).await;

    HttpResponse::Created().json(serde_json::json!({
        "status": "Ok",
        "data": user
    }))
}

async fn insert_user(username: &str, pool: &PgPool) -> Users {
    sqlx::query_as!(
        Users,
        r#"
    INSERT INTO users (
        name, create_timestamp, update_timestamp
    )
    VALUES (
        $1, current_timestamp, current_timestamp
    )
    RETURNING
        id, name, create_timestamp, update_timestamp
    ;
        "#,
        username,
    )
    .fetch_one(pool)
    .await
    .unwrap() // FIXME
}

// `GET /users/{user_id}`
pub async fn get_users(user_id: web::Path<i64>, pool: web::Data<PgPool>) -> HttpResponse {
    let user_id = user_id.into_inner();

    let user_info = get_user_info_by_id(user_id, &pool).await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "Ok",
        "data": user_info
    }))
}

async fn get_user_info_by_id(user_id: i64, pool: &PgPool) -> Users {
    sqlx::query_as!(
        Users,
        r#"
    SELECT
        id, name, create_timestamp, update_timestamp
    FROM
        users
    WHERE
        id = $1
    ;
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .unwrap() // FIXME
}

// `PATCH /users/{user_id}`
pub async fn patch_users(
    user_id: web::Path<i64>,
    json: web::Json<Body>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let user_id = user_id.into_inner();
    let new_username = &*json.name;

    update_username(user_id, new_username, &pool).await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "Ok",
    }))
}

async fn update_username(user_id: i64, new_username: &str, pool: &PgPool) {
    sqlx::query!(
        r#"
    UPDATE
        users
    SET
        name = $1, update_timestamp = current_timestamp
    WHERE
        id = $2
    ;
        "#,
        new_username,
        user_id,
    )
    .execute(pool)
    .await
    .unwrap(); // FIXME
}

// `DELETE /users/{user_id}`
pub async fn delete_users(user_id: web::Path<i64>, pool: web::Data<PgPool>) -> HttpResponse {
    let user_id = user_id.into_inner();

    delete_user_by_id(user_id, &pool).await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "Ok",
    }))
}

async fn delete_user_by_id(user_id: i64, pool: &PgPool) {
    sqlx::query!(
        r#"
    DELETE FROM
        users
    WHERE
        id = $1
    ;
        "#,
        user_id,
    )
    .execute(pool)
    .await
    .unwrap(); // FIXME
}
