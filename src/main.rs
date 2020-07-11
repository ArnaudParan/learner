use std::sync::{Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result, get, put};
use actix_files::NamedFile;
use actix_files as fs;
use sqlite;
use sqlite::State;
use serde::Deserialize;
use serde_json::json;
use rand;
use rand::prelude::*;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::prelude::*;

static UNSAFE_DB_PATH : &str = "db";
lazy_static! {
    static ref SAFE_DB_PATH: Mutex<&'static str> = Mutex::new(UNSAFE_DB_PATH);
}

// TODO allow only valid testid
#[get("/test/{testid}")]
async fn test_endpoint(_testid: web::Path<u32>) -> Result<NamedFile> {
    Ok(NamedFile::open("frontend/dist/test.html")?)
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("frontend/dist/index.html")?)
}

#[derive(Deserialize)]
struct TestId {
    testid: i64,
}

#[get("/api/test")]
async fn api_test(test_info: web::Query<TestId>) -> Result<impl Responder> {
    let db_path = SAFE_DB_PATH.lock().unwrap();
    let conn = sqlite::open(*db_path).unwrap();
    let mut req_file = File::open("get_test.sql")?;
    let mut req = String::new();
    req_file.read_to_string(&mut req)?;
    let mut statement = conn
        .prepare(req)
        .unwrap();

    statement.bind(1, test_info.testid).unwrap();

    let res: String;

    // TODO mix the data
    if let State::Row = statement.next().unwrap() {
        let labels = ["original", "translation", "phonetic"];
        let mut rng = rand::thread_rng();

        if let Ok(_) = statement.read::<String>(3) {
            let mut ids: Vec<usize> = (0..3).collect();
            ids.shuffle(&mut rng);

            res = json!({
                "id": statement.read::<i64>(0).unwrap(),
                "data": [
                {"val": statement.read::<String>(ids[0] + 1).unwrap(), "label": labels[ids[0]]},
                {"val": statement.read::<String>(ids[1] + 1).unwrap(), "label": labels[ids[1]]},
                {"val": statement.read::<String>(ids[2] + 1).unwrap(), "label": labels[ids[2]]},
                ]
            })
            .to_string();
        }
        else {
            let mut ids: Vec<usize> = (0..2).collect();
            ids.shuffle(&mut rng);

            res = json!({
                "id": statement.read::<i64>(0).unwrap(),
                "data": [
                {"val": statement.read::<String>(ids[0] + 1).unwrap(), "label": labels[ids[0]]},
                {"val": statement.read::<String>(ids[1] + 1).unwrap(), "label": labels[ids[1]]},
                ]
            })
            .to_string();
        }
    }
    else {
        res = String::from("{\"err\": \"No data found\"}");
    }

    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(res))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestRes {
    label: String,
    valid_answer: bool,
}

#[derive(Deserialize)]
struct TestResults {
    testid: i32,
    data: Vec<TestRes>,
}

#[put("/api/test_result")]
async fn api_test_result(results: web::Json<TestResults>) -> impl Responder {
    let db_path = SAFE_DB_PATH.lock().unwrap();
    let conn = sqlite::open(*db_path).unwrap();
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let mut original = sqlite::Value::Null;
    let mut translation = sqlite::Value::Null;
    let mut phonetic = sqlite::Value::Null;
    for res in results.data.iter() {
        match res.label.as_ref() {
            "original" => original = sqlite::Value::Integer(res.valid_answer as i64),
            "translation" => translation = sqlite::Value::Integer(res.valid_answer as i64),
            "phonetic" => phonetic = sqlite::Value::Integer(res.valid_answer as i64),
            _ => ()
        }
    }
    let mut cursor = conn.prepare("INSERT INTO test_result (original, translation, phonetic, created_at, test) VALUES (?, ?, ?, ?, ?)").unwrap().cursor();
    cursor.bind(&[
                original,
                translation,
                phonetic,
                sqlite::Value::Integer(created_at as i64),
                sqlite::Value::Integer(results.testid as i64)
    ]).unwrap();
    cursor.next().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"data\":\"ok\"}")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api_test)
            .service(api_test_result)
            .service(index)
            .service(test_endpoint)
            .service(fs::Files::new("/static", "frontend/dist").show_files_listing())
    })
    .bind("127.0.0.1:8088")?
        .run()
        .await
}
