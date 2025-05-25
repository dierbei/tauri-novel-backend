use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, Result, delete, get, post, put, web};

use actix_web::middleware::Logger;
use env_logger::Env;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Iden};

mod response;
mod users;

use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use users::Entity as User;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterUserReq {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterUserRes {
    pub code: u32,
    pub data: Data,
    pub msg: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub token: String,
    pub user: UserRes,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRes {
    pub id: String,
    pub email: String,
    pub created_at: String,
    pub avatar_url: Option<String>,
}

async fn register(
    state: web::Data<AppState>,
    req: web::Json<RegisterUserReq>,
) -> Result<impl Responder> {
    // let db = &state.db;
    //
    // let user_active_model = users::ActiveModel {
    //     id: NotSet,
    //     name: Set(req.name.clone()),
    // };
    //
    // let res = user_active_model.insert(db).await.expect("插入失败");

    info!("Register user: {:?}", req);

    Ok(web::Json(response::success( Data {
        // todo jwt
        token: "".to_string(),
        user: UserRes {
            id: "1".to_string(),
            email: "xiaolatiao@qq.com".to_string(),
            created_at: "2025-06-14 12:04:30".to_string(),
            avatar_url: Some(
                "https://www.zmtc.com/wp-content/uploads/2023/0308/20230308091804414.jpg"
                    .to_string(),
            ),
        },
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let db: DatabaseConnection =
        Database::connect("postgres://postgres:123456@localhost:5432/tauri_test")
            .await
            .expect("初始化数据库连接失败");
    let state = AppState { db };

    HttpServer::new(move || {
        let scope = web::scope("/api").service(
            web::scope("/auth").route("/register", web::post().to(register)), // todo
        );

        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .service(scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
