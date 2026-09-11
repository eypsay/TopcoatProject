mod books;
mod layout;
mod component;
mod pages;

use topcoat::{Result,
    router::{Router, RouterBuilderDiscoverExt,content::Json,route}};
use topcoat::router::error::RouterErrorExt;
use crate::books::Book;

#[route(GET "/api/health")]
async fn health() -> Result<&'static str> {
    Ok("ok")
}

#[route(GET "/api/books")]
async fn api_books() -> Result<Json<Vec<Book>>> {
    Ok(Json(books::all()))
}
#[tokio::main]
async fn  main() {
    topcoat::start(Router::builder().discover().build()).await.unwrap();

    //println!("Hello, world!");
}



