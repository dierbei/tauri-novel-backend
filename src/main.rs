use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

use actix_web::middleware::Logger;
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection, EntityTrait};

mod users;
use users::Entity as User;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[get("/users")]
async fn list(state: web::Data<AppState>)  -> Result<impl Responder> {
    let db = &state.db;
    let data = list_user(db).await.expect("查询失败");

    Ok(web::Json(data))
}

async fn list_user(db: &DatabaseConnection) -> Result<Vec<users::Model>, sea_orm::DbErr> {
    let data = User::find().all(db).await?;
    Ok(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db: DatabaseConnection = Database::connect("postgres://postgres:123456@localhost:5432/tauri_test").await.expect("初始化数据库连接失败");
    let state = AppState{ db };


    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(state.clone()))
        .wrap(Logger::default())
        .service(list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}