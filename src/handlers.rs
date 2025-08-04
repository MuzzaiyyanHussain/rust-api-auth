use actix_web::{HttpResponse, Responder, web};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::auth::{generate_jwt, hash_password, verify_password};
use crate::models::{LoginInput, RegisterInput, User};

static USERS: Lazy<Mutex<HashMap<String, User>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn register(form: web::Json<RegisterInput>) -> impl Responder {
    let mut users = USERS.lock().unwrap();

    if users.contains_key(&form.email) {
        return HttpResponse::BadRequest().body("User already exists");
    }

    let user = User {
        id: Uuid::new_v4(),
        email: form.email.clone(),
        password_hash: hash_password(&form.password),
    };

    users.insert(form.email.clone(), user.clone());
    HttpResponse::Ok().json(user)
}

pub async fn login(form: web::Json<LoginInput>) -> impl Responder {
    let users = USERS.lock().unwrap();

    if let Some(user) = users.get(&form.email) {
        if verify_password(&user.password_hash, &form.password) {
            let token = generate_jwt(&user.id.to_string());
            return HttpResponse::Ok().json(serde_json::json!({ "token": token }));
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
