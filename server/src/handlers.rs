use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use share::{FrontUser, LoginUser, RegisterUser, Tags};
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{auth::hash_password, model::User};

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
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO user_tags (user_id, tag_id) ");

    query_builder.push_values(tags.iter(), |mut b, tag_id| {
        b.push_bind(user_id);
        b.push_bind(tag_id);
    });

    // Выполняем запрос
    query_builder
        .build()
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok((StatusCode::OK, Json("Сохранено".to_string())))
}

pub async fn get_all_people(State(pool): State<PgPool>) -> Result<impl IntoResponse, String> {
    let get_users = sqlx::query!("SELECT id, username FROM users;")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let users: Vec<User> = get_users
        .into_iter()
        .map(|record| User {
            id: record.id,
            username: record.username.unwrap(),
            email: String::new(),
            bio: String::new(),
            password_hash: String::new(),
        })
        .collect();

    Ok(Json(users))
}

pub async fn get_people_by_name(
    State(pool): State<PgPool>,
    Json(email): Json<String>,
) -> Result<impl IntoResponse, String> {
    let mut search_by_email = String::from("%");
    search_by_email.push_str(&email);
    search_by_email.push_str("%");

    let get_users = sqlx::query!(
        "SELECT id, username FROM users WHERE email ILIKE $1;",
        search_by_email
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let users: Vec<User> = get_users
        .into_iter()
        .map(|record| User {
            id: record.id,
            username: record.username.unwrap(),
            email: String::new(),
            bio: String::new(),
            password_hash: String::new(),
        })
        .collect();

    Ok(Json(users))
}

pub async fn add_friend(
    State(pool): State<PgPool>,
    Json((email, friend_id)): Json<(String, i32)>,
) -> Result<impl IntoResponse, String> {
    let user_id = sqlx::query!("SELECT id FROM users WHERE email = $1;", email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if user_id.is_none() {
        return Ok((StatusCode::NOT_FOUND, Json("Увы".to_string())));
    }

    let _user = sqlx::query!(
        "INSERT INTO user_subscribers (user_id, subscriber_id) VALUES ($1, $2);",
        user_id.unwrap().id,
        friend_id,
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok((StatusCode::OK, Json("Успешно".to_string())))
}

pub async fn get_friends(
    State(pool): State<PgPool>,
    Json(email): Json<String>,
) -> Result<impl IntoResponse, String> {
    let user_id = sqlx::query!("SELECT id FROM users WHERE email = $1;", email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if user_id.is_none() {
        return Ok((StatusCode::NOT_FOUND, Json(Vec::<FrontUser>::new())));
    }

    let friends_ids = sqlx::query!(
        "SELECT subscriber_id FROM user_subscribers WHERE user_id = $1;",
        user_id.unwrap().id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Собираем список id друзей в вектор
    let friend_ids_vec: Vec<i32> = friends_ids.iter().map(|f| f.subscriber_id).collect();

    if friend_ids_vec.is_empty() {
        // Возвращаем пустой список, если друзей нет
        return Ok((StatusCode::OK, Json(Vec::<FrontUser>::new())));
    }

    // Запрос для получения данных по списку id друзей
    let friends_data = sqlx::query!(
        "SELECT id, username, email, bio FROM users WHERE id = ANY($1)",
        &friend_ids_vec
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok((
        StatusCode::OK,
        Json(
            friends_data
                .iter()
                .map(|data| FrontUser {
                    id: data.id,
                    username: data.username.clone().unwrap(),
                    email: data.email.clone().unwrap(),
                    bio: data.bio.clone().unwrap(),
                })
                .collect(),
        ),
    ))
}
