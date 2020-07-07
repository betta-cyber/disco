#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use simple_redis::{Interrupts, Message};
use serde_json::{Result, Value};

mod mail;

fn main() {
    pretty_env_logger::init();

    match simple_redis::create("redis://:root@127.0.0.1:6379/") {
        Ok(mut client) =>  {
           println!("Created Redis Client");

            let mut result = client.subscribe("notify");
            assert!(result.is_ok());

            // fetch messages from all subscriptions
            client.fetch_messages(
                &mut |message: Message| -> bool {
                    let payload : String = message.get_payload().unwrap();
                    let v: Value = match serde_json::from_str(&payload) {
                        Err(e) => Value::Null,
                        Ok(v) => {
                            println!("Got message: {:#?}", v);
                            v
                        }
                    };

                    if v["type"] == "email" {
                        mail::send_mail(
                            &v["sender"].as_str().unwrap(),
                            &v["receiver"].as_str().unwrap(),
                            &v["cc"].as_str().unwrap(),
                            &v["data"]["subject"].as_str().unwrap(),
                            &v["data"]["content"].as_str().unwrap()
                        );
                    }

                    // continue fetching
                    false
                },
                // interrupts enable you to break the fetching blocking call
                &mut || -> Interrupts { Interrupts::new() },
            ).unwrap();
        },
        Err(error) => println!("Unable to create Redis client: {}", error)
    }
    // mail::send_mail();
}

// use argon2::{self, Config};
// use rand::Rng;
// use serde::Deserialize;
// use std::collections::HashMap;
// use std::sync::Arc;
// use tokio::sync::Mutex;
// use warp::{http::StatusCode, Filter};

// #[derive(Debug, Deserialize)]
// struct User {
    // username: String,
    // password: String,
// }

// #[tokio::main]
// async fn main() {
    // let db = Arc::new(Mutex::new(HashMap::<String, User>::new()));
    // let db = warp::any().map(move || Arc::clone(&db));

    // let register = warp::post()
        // .and(warp::path("register"))
        // .and(warp::body::json())
        // .and(db.clone())
        // .and_then(register);
    // let login = warp::post()
        // .and(warp::path("login"))
        // .and(warp::body::json())
        // .and(db.clone())
        // .and_then(login);

    // let routes = register.or(login);
    // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
// }

// async fn register(
    // new_user: User,
    // db: Arc<Mutex<HashMap<String, User>>>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
    // let mut users = db.lock().await;
    // if users.contains_key(&new_user.username) {
        // return Ok(StatusCode::BAD_REQUEST);
    // }
    // let hashed_user = User {
        // username: new_user.username,
        // password: hash(new_user.password.as_bytes()),
    // };
    // users.insert(hashed_user.username.clone(), hashed_user);
    // Ok(StatusCode::CREATED)
// }

// async fn login(
    // credentials: User,
    // db: Arc<Mutex<HashMap<String, User>>>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
    // let users = db.lock().await;
    // match users.get(&credentials.username) {
        // None => Ok(StatusCode::BAD_REQUEST),
        // Some(user) => {
            // if verify(&user.password, credentials.password.as_bytes()) {
                // Ok(StatusCode::OK)
            // } else {
                // Ok(StatusCode::UNAUTHORIZED)
            // }
        // }
    // }
// }

// pub fn hash(password: &[u8]) -> String {
    // let salt = rand::thread_rng().gen::<[u8; 32]>();
    // let config = Config::default();
    // argon2::hash_encoded(password, &salt, &config).unwrap()
// }

// pub fn verify(hash: &str, password: &[u8]) -> bool {
    // argon2::verify_encoded(hash, password).unwrap_or(false)
// }
