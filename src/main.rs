
use actix_web::{get,App,HttpResponse,HttpServer,Responder,web};
use serde::Deserialize;
// How to extract the data from the URl

// Intro to - web::Query , web::Path 
#[derive(Deserialize)]
struct GreetingParams {
    name: Option<String>,
    lang: Option<String>
}

#[get("/greet_query")]
async fn greet_query(query: web::Query<GreetingParams>) -> impl Responder {
    let name = query.name.as_deref().unwrap_or("Guest");
    let lang = query.lang.as_deref().unwrap_or("English");
    HttpResponse::Ok().body(format!("Hello {} , You prefer {}",name,lang))
}

#[get("greet_path/{name}")]
async fn greet_path(query: web::Path<String>) -> impl Responder{
    let name = query.into_inner();
    HttpResponse::Ok().body(format!("Hello {}! ,from the parameters",name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(greet_path)
        .service(greet_query)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}