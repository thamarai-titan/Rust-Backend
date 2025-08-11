use std::{sync::Mutex};
use actix_web::{App,web,HttpResponse,HttpServer,Responder};
use serde::{Deserialize,Serialize};
use uuid::Uuid;
use actix_cors::Cors;

struct AppState{
    todos: Mutex<Vec<TodoItem>>
}

#[derive(Deserialize,Serialize,Debug,Clone)]
struct TodoItem{
    id: String,
    todo: String,
    description: Option<String>,
    done: bool
}

#[derive(Deserialize,Debug)]
struct NewTodo{
    todo: String,
    description: Option<String>
}

async fn get_all_todo(data: web::Data<AppState>)->impl Responder{
    let todos = data.todos.lock().unwrap();
    web::Json(todos.iter().cloned().collect::<Vec<TodoItem>>())
}



async fn get_todo_by_id(data:web::Data<AppState>,path:web::Path<String>)->impl Responder{
    let id = path.into_inner();
    let todos = data.todos.lock().unwrap();

    let todo = todos.iter().find(|todo|  todo.id == id).cloned();
    web::Json(todo)
   
}


async fn create_todo(data:web::Data<AppState>,new_todo: web::Json<NewTodo>)-> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = Uuid::new_v4().to_string();

    let todoitem = TodoItem{
        id:id,
        todo:new_todo.todo.clone(),
        description:new_todo.description.clone(),
        done:false
    };

    todos.push(todoitem.clone());

    HttpResponse::Created().json(todoitem)

}

async fn delete_todo_id(data: web::Data<AppState>,path:web::Path<String>)->impl Responder{
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    if let Some(value) = todos.iter().position(|todo| todo.id == id) {
        todos.swap_remove(value);
        HttpResponse::Ok().body("todo was deleted")
    }
    else {
        HttpResponse::NotFound().body("the todo was not found")
    }
}

async fn modify_todo(data: web::Data<AppState>,path:web::Path<String>,new_todo: web::Json<NewTodo>)->impl Responder{
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    if let Some(val) =todos.iter_mut().find(|todo| todo.id==id)  {
        val.todo = new_todo.todo.clone();
        val.description = new_todo.description.clone();
        HttpResponse::Ok().json(val)
    }
    else{
        HttpResponse::NotFound().body("There is no todo")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let app_state = web::Data::new(AppState{
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
        .wrap(Cors::default().allow_any_header().allow_any_method().allow_any_origin())
        .app_data(app_state.clone())
        .service(web::resource("/todos")
                .route(web::post().to(create_todo))
                .route(web::get().to(get_all_todo)))
        .service(web::resource("/todos/{id}")
                .route(web::get().to(get_todo_by_id))
                .route(web::delete().to(delete_todo_id))
                .route(web::patch().to(modify_todo)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}