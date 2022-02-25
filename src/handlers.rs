pub use crate::prelude::*;

use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/foo/{id}")]
pub async fn get_foo(
    Path(id): Path<String>,
    foo_store: Data<FooStore>,
) -> impl actix_web::Responder {
    log::info!("GET /foo/{:#?}", id);
    match foo_store.get_foo(id) {
        Ok(foo) => {
            let body: String = serde_json::to_string(&foo).unwrap();
            HttpResponse::Ok().body(body)
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/foo")]
pub async fn post_foo(foo: Json<Foo>, foo_store: Data<FooStore>) -> impl actix_web::Responder {
    log::info!("POST /foo/{:#?}", foo);
    let name = match &foo.name {
        Some(name) => name.clone(),
        None => return HttpResponse::BadRequest().body("No name provided"),
    };
    match foo_store.put_foo(Foo::new(name)) {
        Ok(foo) => {
            let body: String = serde_json::to_string(&foo).unwrap();
            HttpResponse::Ok().body(body)
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[delete("/foo/{id}")]
pub async fn delete_foo(
    Path(id): Path<String>,
    foo_store: Data<FooStore>,
) -> impl actix_web::Responder {
    log::info!("DELETE /foo/{:#?}", id);
    match foo_store.delete_foo(id) {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
