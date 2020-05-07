use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result, get, put};
use actix_files::NamedFile;
use actix_files as fs;
use sqlite;
use sqlite::State;
use serde::Deserialize;
use serde_json::json;
use rand;
use rand::prelude::*;

static DB_PATH : &str = "db";

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
async fn api_test(test_info: web::Query<TestId>) -> impl Responder {
    let conn = sqlite::open(DB_PATH).unwrap();
    let mut statement = conn
        .prepare("SELECT id, original, phonetic, translation FROM test WHERE test.testid = ? ORDER BY RANDOM() LIMIT 1;")
        .unwrap();

    statement.bind(1, test_info.testid).unwrap();

    let res: String;

    // TODO mix the data
    if let State::Row = statement.next().unwrap() {
        let mut ids: Vec<usize> = (0..3).collect();
        let mut rng = rand::thread_rng();
        ids.shuffle(&mut rng);

        let labels = ["original", "phonetic", "translation"];
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
        res = String::from("{\"err\": \"No data found\"}");
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(res)
}

#[put("/api/test_result")]
async fn api_test_result() -> impl Responder {
    let conn = sqlite::open(DB_PATH).unwrap();
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
