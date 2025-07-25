
use actix_web::{App,HttpResponse,HttpServer,Responder,web};
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct FormData{
    name: String,
    age: u32
}

async fn handle_form_data(fdata: web::Form<FormData>) -> impl Responder{
    println!("The data was received {:?}",fdata);

    HttpResponse::Ok().body(format!("Your name and age is {} - {}",fdata.name,fdata.age))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(web::resource("/contact").route(web::post().to(handle_form_data)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}