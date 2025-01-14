use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use rand_core::OsRng;
use std::sync::Arc;

use crate::{
    repositories::admin_user_repository::AdminUser,
    requests::admin_user_create_request::CreateAdminUserRequest, routes::{AppState, establish_connection},
};

pub async fn create_admin_user_handler(
    Extension(current_user): Extension<AdminUser>,
    app_state: State<Arc<AppState>>,
    Json(payload): Json<CreateAdminUserRequest>,
) -> impl IntoResponse {
    let password = payload.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let connection = establish_connection().await;

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let admin_user = app_state
        .admin_user_repository
        .create(
            connection,
            payload.name,
            payload.email,
            password_hash,
            current_user.email,
        )
        .await;

    Json(admin_user).into_response()
}
