mod books;
mod layout;
mod component;
mod pages;

use topcoat::{
    asset::{AssetBundle,RouterBuilderAssetExt},
    Result,
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
    topcoat::start(Router::builder().discover().
        assets(AssetBundle::load().unwrap()).
        build()).await.unwrap();

    //println!("Hello, world!");
}



