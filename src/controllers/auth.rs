
use crate::models::auth::{User, Address};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use axum::Json;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::env;

pub async fn get_users() -> Json<Vec<User>> {
    
    let mut users = vec![];
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = Arc::new(PgPoolOptions::new()
        .max_connections(2)
        .connect(&durl)
        .await
        .expect("unable to connect to database"));

    let rows = sqlx::query!("SELECT id,name, email FROM users")
        .fetch_all(&*pool)
        .await
        .unwrap();

    for row in rows {
        let user_id: i32 = row.id;
        let name: String = row.name.unwrap_or_else(|| "".to_string());
        let email: String = row.email.unwrap_or_else(|| "".to_string());

        let address_rows = sqlx::query!("SELECT user_id, address1, pincode FROM addresses WHERE user_id = $1", user_id)
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Path;
    use axum::response::Json;
    use sqlx::postgres::PgPoolOptions;
    use std::ffi::OsString;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_get_user_by_id() {
        // Create a test database URL
        let value = OsString::from("postgres://tiwhlrho:gDCWUAftUlKyRtQakEzpwr4DQN6lL209@berry.db.elephantsql.com/tiwhlrho");
        let value_str = value.into_string().unwrap();
        env::set_var("DATABASE_URL", &value_str);

        let pool = Arc::new(
            PgPoolOptions::new()
                .max_connections(2)
                .connect(&value_str)
                .await
                .expect("unable to connect to database"),
        );

        sqlx::query!(
            "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)",
            11,
            "Priya3",
            "Priya3@example.com"
        )
        .execute(&*pool)
        .await
        .expect("unable to insert user");

        sqlx::query!(
            "INSERT INTO addresses (user_id, address1, pincode) VALUES ($1, $2, $3)",
            11,
            "456 Main St",
            "4567"
        )
        .execute(&*pool)
        .await
        .expect("unable to insert address");

        let id = Path(11);
        let response = get_user_by_id(id).await;

        let expected_user = User {
            name: "Priya3".to_string(),
            email: "Priya3@example.com".to_string(),
            address: vec![Address {
                address1: "456 Main St".to_string(),
                pincode: "4567".to_string(),
            }],
        };

        assert_eq!(response.name, "Priya3");
    }
}


pub async fn get_user_by_id(axum::extract::Path(id): axum::extract::Path<i32>) -> Json<User> {
    
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = Arc::new(PgPoolOptions::new()
        .max_connections(2)
        .connect(&durl)
        .await
        .expect("unable to connect to database"));

    let row = sqlx::query!("SELECT id,name,email FROM users where id = $1", id)
        .fetch_one(&*pool)
        .await
        .unwrap();


    let user_id: i32 = row.id;
    let name: String = row.name.unwrap_or_else(|| "".to_string());
    let email: String = row.email.unwrap_or_else(|| "".to_string());

    let address_rows = sqlx::query!("SELECT user_id, address1, pincode FROM addresses WHERE user_id = $1", user_id)
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
        .max_connections(2)
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

