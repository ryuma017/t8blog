use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct Body {
    name: String,
}

#[derive(serde::Deserialize)]
pub struct Users {
    id: i64,
    name: String,
    create_timestamp: NaiveDateTime,
    update_timestamp: NaiveDateTime,
}

pub async fn create_user(json: web::Json<Body>, pool: web::Data<PgPool>) -> HttpResponse {
    let user = insert_user(&pool, &json).await;

    HttpResponse::Created().json(serde_json::json!({
        "status": "Ok",
        "data": {
            "id": user.id,
            "name": user.name,
            "create_timestamp": user.create_timestamp,
            "update_timestamp": user.update_timestamp
        }
    }))
}

async fn insert_user(pool: &PgPool, body: &Body) -> Users {
    let username = &body.name;

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
