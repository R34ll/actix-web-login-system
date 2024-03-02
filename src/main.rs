mod database;
mod routes;

use database::Database;

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn not_found(req: HttpRequest) -> impl Responder {
    println!("Page was not found: {:?}", req.path());
    HttpResponse::NotFound().body("Page was not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Database
    let db = Database::init()
        .await
        .expect("Error connecting to database");
    
    let db_data = web::Data::new(db);

    // Session
    let secret_key = Key::generate();

    let redis_session = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) 
            .wrap(SessionMiddleware::new(redis_session.clone(), secret_key.clone()))
            .route("/", web::get().to(routes::home))
            .route("/home", web::get().to(routes::home))
            .service(
                web::scope("/account")
                    .service(routes::account::login_get)
                    .service(routes::account::login_post)
                    .service(routes::account::register_get)
                    .service(routes::account::register_post)
                    .service(routes::account::profile_get)
                    .service(routes::account::logout), 
            )
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
