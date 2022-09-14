use actix_cors::Cors;
use actix_web::{
    error, get, post,
    web::{self, Json},
    App, HttpResponse, HttpServer,
};
use chrono::{self, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::{process, sync::Arc};
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

#[derive(Deserialize)]
struct Paste {
    title: String,
    body: String,
}

#[derive(Serialize, Debug)]
struct PasteResponse {
    title: String,
    body: String,
    hash: String,
    creation_date: Option<chrono::DateTime<Utc>>,
    click_count: i32,
}

impl std::fmt::Display for PasteResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "title: {}, hash: {}, body: {}",
            self.title,
            self.hash,
            if &self.body.chars().count() > &60usize {
                String::from(&self.body).split_off(60)
            } else {
                String::from(&self.body)
            }
        )
    }
}

#[derive(Serialize)]
struct PasteResponses {
    pastes: Vec<PasteResponse>,
}

#[post("/api/v1/create")]
async fn create_paste(
    paste: web::Json<Paste>,
    db_client: web::Data<Arc<Mutex<Client>>>,
) -> Result<Json<PasteResponse>, actix_web::Error> {
    eprintln!("/create was called");
    let rng = rand::thread_rng();
    let random_hash: String = rng
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let cur_datetime = chrono::Utc::now();

    let rows = match db_client
        .clone()
        .lock()
        .await
        .execute(
            "INSERT INTO pastes (uniqueHash, title, data, creation_date) VALUES ($1, $2, $3, $4)",
            &[
                &random_hash.clone(),
                &paste.title,
                &paste.body,
                &cur_datetime,
            ],
        )
        .await
    {
        Ok(_) => Ok(PasteResponse {
            title: String::from(&paste.title),
            body: String::from(""),
            hash: String::from(&random_hash),
            creation_date: Some(cur_datetime),
            click_count: 0,
        }),
        Err(err) => Err(format!("Failed to create entry: {}", err)),
    };

    match rows {
        Ok(paste) => {
            eprintln!(
                "Title: {}\nDate: {}\nBody:{}",
                &paste.title, cur_datetime, paste.body
            );
            Ok(Json(paste))
        }
        Err(msg) => Err(actix_web::error::ErrorInternalServerError(msg)),
    }
}

#[get("/api/v1/pastes")]
async fn get_pastes(
    db_client: web::Data<Arc<Mutex<Client>>>,
) -> Result<Json<PasteResponses>, actix_web::Error> {
    eprintln!("/pastes was called");
    let mut pastes = Vec::new();
    match db_client
        .clone()
        .lock()
        .await
        .query(
            "SELECT uniqueHash, title, creation_date, click_count
                FROM pastes
                ORDER BY creation_date",
            &[],
        )
        .await
    {
        Ok(rows) => {
            eprintln!("Received rows: {}", rows.len());
            rows.iter().for_each(|row| {
                let res = PasteResponse {
                    hash: row.get(0),
                    title: row.get(1),
                    body: String::from(""),
                    creation_date: row.try_get(2).unwrap_or(None),
                    click_count: row.try_get(3).unwrap_or(0),
                };
                pastes.push(res)
            });
            Ok(Json(PasteResponses { pastes }))
        }
        Err(err) => {
            eprintln!("Error loading /pastes: {}", err);
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}
async fn increase_click_count(db_client: Arc<Mutex<Client>>, hash: &String) -> u64 {
    let hash = String::from(hash);
    db_client
        .lock()
        .await
        .execute(
            "UPDATE pastes
            SET click_count = click_count+1
            WHERE uniqueHash = $1",
            &[&hash],
        )
        .await
        .expect("increase_click_count failed")
}

#[get("/api/v1/paste/{hash}")]
async fn get_paste(
    hash: web::Path<String>,
    db_client: web::Data<Arc<Mutex<Client>>>,
) -> Result<Json<PasteResponse>, actix_web::error::Error> {
    println!("/paste/{hash} was called");
    let response: PasteResponse;
    let hash2 = String::clone(&hash);
    let affected_rows = db_client
        .lock()
        .await
        .query(
            "SELECT uniqueHash, title, data, creation_date, click_count
                FROM pastes
                WHERE uniqueHash = $1
                LIMIT 1",
            &[&hash.to_string()],
        )
        .await
        .expect("Failed to insert paste into database");

    let row = &affected_rows[0];
    response = PasteResponse {
        hash: row.get(0),
        title: row.get(1),
        body: row.get(2),
        creation_date: row.try_get(3).unwrap_or(None),
        click_count: row.try_get(4).unwrap_or(0),
    };
    increase_click_count(Arc::clone(&db_client), &hash2).await;
    Ok(Json(response))
}

async fn create_db_client(
    config: String,
) -> Result<Arc<Mutex<Client>>, tokio_postgres::error::Error> {
    let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;

    tokio::spawn(async move {
        // Opens connection and handles error if if crashes
        // https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Connection.html
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // return client
    Ok(Arc::new(Mutex::new(client)))
}

fn get_database_config() -> String {
    // TODO: Move into some kind of .env file
    let postgres_database_name = "paste_db_new";
    let postgres_username = "db_user";
    let postgres_hostname = "localhost";
    let postgres_port = "5432";
    let postgres_password = "userpassword123";

    format!(
        "host={} port={} user={} dbname={} password={}",
        postgres_hostname,
        postgres_port,
        postgres_username,
        postgres_database_name,
        postgres_password
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hostname = "127.0.0.1";
    let port: u16 = 9095;

    let config = get_database_config();

    eprintln!("Trying to connect to database...");
    let client = match create_db_client(config).await {
        Ok(client) => {
            eprintln!("Connected to database.");
            client
        }
        Err(err) => {
            eprintln!("Error while connecting to databse: {}", err);
            process::exit(1);
        }
    };

    eprintln!("Trying to create database table");

    let a = client.clone();
    let db_lock = a.lock().await;
    let result = db_lock
        .execute(
            "CREATE TABLE IF NOT EXISTS pastes (
                    id      SERIAL PRIMARY KEY,
                    uniqueHash TEXT NOT NULL,
                    title    TEXT NOT NULL,
                    data    TEXT NOT NULL,
                    creation_date TIMESTAMP WITH TIME ZONE,
                    click_count INTEGER NOT NULL DEFAULT 0)",
            &[],
        )
        .await;

    match result {
        Ok(rows) => {
            eprintln!("New Tables created: {}", rows);
        }
        Err(err) => {
            eprintln!("Error while creating table: {}", err);
        }
    }
    // manually release lock here, since the lifetime of db_lock exceeds the listening server's lifetime and therefore would keep the lock permanently
    drop(db_lock);

    eprintln!(
        "Starting backend server. Listening on {}:{}",
        &hostname, &port
    );

    HttpServer::new(move || {
        //TODO: Figure out why I can't move 'json_config' and 'cors' out of this thread and closure
        let database_client = Arc::clone(&client);

        let json_config = web::JsonConfig::default()
            .limit(4000)
            .error_handler(|err, _req| {
                //create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(json_config)
            .app_data(web::Data::new(database_client))
            .wrap(cors)
            .route("api/v1/hello", web::get().to(|| async { "Hello World!" }))
            .service(get_paste)
            .service(create_paste)
            .service(get_pastes)
    })
    .bind((hostname, port))
    .expect("Failed to start server")
    .run()
    .await
}
