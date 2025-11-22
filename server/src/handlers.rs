use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use share::{LoginUser, RegisterUser, Tags};
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{auth::hash_password};

pub async fn register(
    State(pool): State<PgPool>,
    Json(user_data): Json<RegisterUser>,
) -> Result<impl IntoResponse, String> {
    let existing = sqlx::query!("SELECT id FROM users WHERE email = $1", user_data.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if existing.is_some() {
        return Ok((StatusCode::CONFLICT, Json("Увы".to_string())));
    }

    let password_hash = hash_password(&user_data.password);

    let _user = sqlx::query!(
        "INSERT INTO users (username, email, bio, password_hash) VALUES ($1, $2, $3, $4);",
        user_data.username,
        user_data.email,
        " ",
        password_hash
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok((StatusCode::OK, Json(user_data.email)))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(user_data): Json<LoginUser>,
) -> Result<impl IntoResponse, String> {
    let existing = sqlx::query!(
        "SELECT id FROM users WHERE email = $1 AND password_hash = $2;",
        user_data.email,
        hash_password(&user_data.password)
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| e.to_string())?;

    if existing.is_some() {
        return Ok((StatusCode::OK, Json(user_data.email)));
    }

    Ok((StatusCode::NOT_FOUND, Json("Увы".to_string())))
}

pub async fn get_base_tags(State(pool): State<PgPool>) -> Result<impl IntoResponse, String> {
    let get_tags = sqlx::query!("SELECT id, name, custom FROM tags WHERE custom = false;")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let tags: Vec<Tags> = get_tags
        .into_iter()
        .map(|record| Tags {
            id: record.id,
            name: record.name.unwrap(),
            custom: record.custom.unwrap(),
        })
        .collect();

    Ok(Json(tags))
}

pub async fn set_base_tags(
    State(pool): State<PgPool>,
    Json((tags, email)): Json<(Vec<i32>, String)>,
) -> Result<impl IntoResponse, String> {
    let user = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let user_id = if let Some(u) = user {
        u.id
    } else {
        return Ok((StatusCode::NOT_FOUND, Json("Увы".to_string())));
    };

    // Строим запрос для вставки нескольких строк в user_tags
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO user_tags (user_id, tag_id) "
    );

    query_builder.push_values(tags.iter(), |mut b, tag_id| {
        b.push_bind(user_id);
        b.push_bind(tag_id);
    });

    // Выполняем запрос
    query_builder.build()
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok((StatusCode::OK, Json("Сохранено".to_string())))
}
