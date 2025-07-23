
use actix_web::{App,get,post,HttpResponse,HttpServer,Responder,web};
use serde::{Serialize,Deserialize};
use uuid::Uuid; 

#[derive(Deserialize)]
struct PathInfo{
    name: String
}

#[get("/greet/{name}")]
async fn greetwith_path(info: web::Path<PathInfo>) -> impl Responder{
    let name = &info.name;
    HttpResponse::Ok().body(format!("Hello {}",name))
}

#[derive(Deserialize)]
struct QueryParams{
    name: Option<String>,
    lang: Option<String>
}

#[get("/query")]
async fn greetwith_query(info: web::Query<QueryParams>)->impl Responder{
    let name = info.name.as_deref().unwrap_or("Guest");
    let lang = info.lang.as_deref().unwrap_or("English");

    HttpResponse::Ok().body(format!("Hello {} your choosen language is {}",name,lang))
}

// POST request
#[derive(Deserialize,Debug)]
struct NewItem{
    name: String,
    description: String,
}

#[derive(Serialize,Debug)]
struct ResultItem{
    id: String,
    status: String,
}

#[post("/items")]

async fn create_item(item: web::Json<NewItem>)-> impl Responder{
    println!("Received new item {:?}",item);

    let item_id = uuid::Uuid::new_v4();

    let response = ResultItem{
        id: item_id.to_string(),
        status: "created".to_string(),
    };

    HttpResponse::Created().json(response)

}

#[actix_web::main]

async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .service(greetwith_path)
        .service(greetwith_query)
        .service(create_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



