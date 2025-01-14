use crate::{
    repositories::admin_user_repository::AdminUser,
    requests::login_admin_user_request::LoginAdminUserRequest,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use axum_sessions::extractors::WritableSession;
use std::sync::Arc;
use validator::{HasLen, Validate, ValidationErrors, ValidationErrorsKind};

use crate::routes::{establish_connection, AppState};

pub async fn post_admin_login_handler(
    mut session: WritableSession,
    app_state: State<Arc<AppState>>,
    Form(payload): Form<LoginAdminUserRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let validation_error_list = match payload.validate() {
        Ok(_) => ValidationErrors::new(),
        Err(errors) => errors,
    };

    for (field_name, error) in validation_error_list.errors() {
        // let test = validation_error_list.errors();
        // let test= error::add("sdfs");
        match &error {
            ValidationErrorsKind::Field(field_errors) => {
                for field_error in field_errors {
                    let message = match &field_error.message {
                        Some(message) => message,
                        None => continue,
                    };
                    println!("{:?}", message.is_empty());

                    if !message.is_empty() {
                        // let key = field_name.clone();
                        let validation_key = format!("validation_error_{}", field_name);
                        session
                            .insert(&validation_key, message)
                            .expect("Could not store the validation errors into session.");
                    }
                }
                // println!("Error: {:?}", &error);
            }
            ValidationErrorsKind::Struct(_) => continue,
            ValidationErrorsKind::List(_) => continue,
        }
    }
    if validation_error_list.errors().length() > 0 {
        return Err(Redirect::to("/admin/login").into_response());
    }
    // println!(": {:?}", validation_error_list.errors());

    let connection = establish_connection().await;
    let admin_user: entity::admin_users::Model = app_state
        .admin_user_repository
        .find_by_email(connection, payload.email)
        .await;

    let is_valid = match PasswordHash::new(&admin_user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let validation_error = String::from("your email address or password did not match");

        session
            .insert("validation_error_email", validation_error)
            .expect("Could not store the validation errors into session.");

        // session
        // app_state.avored_validation.un
        return Err(Redirect::to("/admin/login").into_response());
    }

    // let jwt_secret = app_state.config.jwt_secret.as_ref();

    // let expiration = Utc::now()
    //     .checked_add_signed(chrono::Duration::minutes(app_state.config.jwt_maxage))
    //     .expect("valid timestamp")
    //     .timestamp();

    let admin_user_model = AdminUser {
        id: admin_user.id,
        name: admin_user.name,
        email: admin_user.email,
        created_at: admin_user.created_at,
        updated_at: admin_user.updated_at,
        created_by: admin_user.created_by,
        updated_by: admin_user.updated_by,
    };

    // Ok(response)

    session
        .insert("logged_in_user", admin_user_model)
        .expect("Could not store the answer.");

    // let data = json!({});

    // let handlebars = &app_state.handlebars;

    // let html = handlebars
    //     .render("auth/login", &data)
    //     .expect("there is an issue while loading the admin template");

    // println!("Redirect to /admin route");

    Ok(Redirect::to("/admin").into_response())
}
