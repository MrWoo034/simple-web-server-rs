pub mod handlers;
pub mod model;

pub mod prelude {
    pub use crate::handlers::{delete_foo, get_foo, post_foo};
    pub use crate::model::{Foo, FooStore};
    pub use serde::{Deserialize, Serialize};
}

use prelude::*;

use actix_web::{web::Data, App};

const ADDRESS: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("Starting simple-web-server");

    let foo_store = Data::new(FooStore::new());
    let _server = actix_web::HttpServer::new(move || {
        App::new()
            .app_data(foo_store.clone())
            .service(get_foo)
            .service(post_foo)
            .service(delete_foo)
    })
    .bind(ADDRESS)?
    .run()
    .await;

    Ok(())
}
