# Rust-Backend
# rust
```
// The basic actix-web server 

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
    
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
# Hour - 1 [Basics of Rust]
```
fn main(){

    // The immutable variable
    let a = 5;
    println!("{}",a);

    // Mutable variable - to make a variable use keyword - mut
    let mut a = 6;
    println!("{}",a);
    a = 7;
    println!("{}",a);

    // Datatypes 
    // Integers - default integer type is i32
    let an_integer: i32 = 34;
    let anothor_integer = 56;
    println!("The two intergers is {} {} ",an_integer,anothor_integer);

    // Floating point - f32 - [single presision] , f64 - [double presision]
    let a_float: f32 = 3.14;
    let another_float = 5.1244;
    println!("The two float numbers are {} {}",a_float,another_float);

    // Booleans - [ ture or false ]
    let is_active = true;
    let is_notactive = false;
    println!("The two boolean variable values {} {} ",is_active,is_notactive);

    // Characters - char[single unicode scalar value]
    let a_char = 'a';
    println!("The char value {}",a_char);

    // Tuples = Grouping values of different types
    let person_info: (String,i32,bool) = (String::from("Thamarai"),20,true);
    println!("Person info: Name : {} , Age : {} , Active: {}",person_info.0,person_info.1,person_info.2);

    // Arrays = Fixed size of list of element n the same type
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Array first element: {}",numbers[0]);

    // Vectors: growable , heap-allocated lists
    let mut vec_list = vec![1,2,3];
    vec_list.push(4);
    println!("{:?}",vec_list);

    // ===Functions===
    greet("Thamarai");


    let result = add_number(2, 5);
    println!("{}",result);

    // Function with block expressions
    let y = {
        let x = 3;
        x + 3
    };
    println!("The block based expression : {}",y);
}

fn greet(name: &str){
    println!("The Name is {}",name);
}

fn add_number(a: i32,b: i32) -> i32{
    a + b
}
```

# Hour - 2  [Creating a simple httpserver]

```
use actix_web::{get,App,HttpResponse,HttpServer,Responder};

#[get("/greet")]
async fn greet()-> impl Responder{
    HttpResponse::Ok().body("Hello from the Actix-web")
}

#[get("/")]
async fn index() -> impl Responder{
    HttpResponse::Ok().body("This is from the / route")
}

#[actix_web::main]
async fn main()->std::io::Result<()> {
HttpServer::new(|| {
    App::new()
    .service(greet)
    .service(index)
})
.bind("127.0.0.1:8080")?
.run()
.await
}
```

# Getting values from the url

```
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
```

# Hour - 3 [request/response using serde,actix_web]

```
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
```

# Hour - 4 [Creating a post request with urlencoded request]

``` 

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

```

The raw data handling request

```


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

async fn handle_raw_data(rawdata: web::Bytes)-> impl Responder{
    println!("The raw data received {:?} and the total bytes {}",rawdata,rawdata.len());

    HttpResponse::Ok().body(format!("The received data {:?}",rawdata.len()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(web::resource("/contact").route(web::post().to(handle_form_data)))
        .service(web::resource("/rawdata").route(web::post().to(handle_raw_data)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

```