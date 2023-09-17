
use crate::models::{User, Address};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use axum::{Json};

pub async fn get_users() -> Json<Vec<User>> {
    
    let mut users = vec![];
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = Arc::new(PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect to database"));

    let rows = sqlx::query!("SELECT * FROM users")
        .fetch_all(&*pool)
        .await
        .unwrap();

    for row in rows {
        let user_id: i32 = row.id;
        let name: String = row.name.unwrap_or_else(|| "".to_string());
        let email: String = row.email.unwrap_or_else(|| "".to_string());

        let address_rows = sqlx::query!("SELECT * FROM addresses WHERE user_id = $1", user_id)
            .fetch_all(&*pool)
            .await
            .unwrap();

        let mut addresses = vec![];

        for address_row in address_rows {
            addresses.push(Address {
                address1: address_row.address1.unwrap_or_else(|| "".to_string()),
                pincode: address_row.pincode.unwrap_or_else(|| "".to_string()),
            });
        }

        users.push(User {
            name,
            email,
            address: addresses,
        });
    }

    Json(users)
}

pub async fn get_user_by_id(axum::extract::Path(id): axum::extract::Path<i32>) -> Json<User> {
    
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = Arc::new(PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect to database"));

    let row = sqlx::query!("SELECT * FROM users where id = $1", id)
        .fetch_one(&*pool)
        .await
        .unwrap();


    let user_id: i32 = row.id;
    let name: String = row.name.unwrap_or_else(|| "".to_string());
    let email: String = row.email.unwrap_or_else(|| "".to_string());

    let address_rows = sqlx::query!("SELECT * FROM addresses WHERE user_id = $1", user_id)
        .fetch_all(&*pool)
        .await
        .unwrap();

    let mut addresses = vec![];

    for address_row in address_rows {
        addresses.push(Address {
            address1: address_row.address1.unwrap_or_else(|| "".to_string()),
            pincode: address_row.pincode.unwrap_or_else(|| "".to_string()),
        });
    }

    Json(User {
        name,
        email,
        address: addresses,
    })
}


pub async fn add_user(user: Json<User>,) -> String {

    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = Arc::new(PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect to database"));

        let mut tx = pool.begin().await.unwrap();
    
        let user_id: i32 = sqlx::query!("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id", user.name, user.email)
        .fetch_one(&mut tx)
        .await
        .unwrap()
        .id;
        
        println!("User ID: {}", user_id);

        tx.commit().await.unwrap();

        for address in &user.address {
            sqlx::query!(
                r#"
                INSERT INTO addresses (user_id, address1, pincode) VALUES ($1, $2, $3)
                "#,
                user_id,
                address.address1,
                address.pincode
            )
            .execute(&*pool)
            .await
            .unwrap();
        }
        "User created successfully!".to_string()
}

