mod books;

use topcoat::{Result,
              router::{Router,RouterBuilderDiscoverExt,page,layout},
              view::{component,view}
};
use topcoat::view::internal::view;
use crate::books::Book;
#[layout("/")]
async fn root_layout(slot:Result) -> Result {
    view!{
        <!DOCTYPE html>
        <html>
            <head>
                <title>"SHELF"</title>
                topcoat::dev::script()
            </head>
            <body>
                <nav>
                    <a href="/">"Shelf"</a>
                    <span style="margin: 0 10px;">" "</span>
                    <a href="/about">"About"</a>
                </nav>
                (slot?)
            </body>
        </html>
    }
}
#[tokio::main]
async fn  main() {
    topcoat::start(Router::builder().discover().build()).await.unwrap();

    //println!("Hello, world!");
}

#[page("/")]
async fn home() -> Result {
    let shelf=books::all();
    view!{
        // <!DOCTYPE html>
        // <html>
        //     <head>
        //         <title>"SHELF"</title>
        //         topcoat::dev::script()
        //     </head>
        //     // <body>hello(name: "EEYP")</body>
        //     <body>
        //         // book_card(title: "sefiller", author: "Dost", year: 2020, finished: true)
        //         // book_card(title: "reziller", author: "hugo", year: 2000, finished: true)
        //         // book_card(title: "filler", author: "elt", year: 2010, finished: false)
        <h1>
            "MY Shelf ("
            (shelf.len())
            " books)"
        </h1>
        for book in shelf {
            book_card(
                title: book.title.as_str(),
                author: book.author.as_str(),
                year: book.year,
                finished: book.finished
            )
        }
        //         </body>
        //     </html>
        // }
    }}
#[page("/about")]
async  fn about()->Result{
    view!{
        <h1>"About Shelf"</h1>
        <p>" small reading list, built in rust with topcoat"</p>
    }
}
// #[component]
// async fn hello(name:&str) -> Result {
//     let book_count=4;
//     let genres= vec!["Science","History","Potery"];
//     let is_logged_in=true;
//     view!{
//         <h1>
//             "Hello,"
//             (name)
//             "!"
//         </h1>
//         // <p>
//         //     "you have "
//         //     (book_count)
//         //     " books on the shelf"
//         // </p>
//         // <a href="/about">"About this site"</a>
//         // <p title=(name)>"Hover me."</p>
//
//         <ul>
//             for genre in &genres {
//                 <li>(genre)</li>
//             }
//         </ul>
//
//         // if is_logged_in {
//         //     <p>"Signed in"</p>
//         // }else{
//         <a
//             href="/login"
//             if is_logged_in {
//                 class="active"
//                 aria-current="page"
//             }
//         >
//             "Sign In!"
//         </a>
//     }
// }

#[component]
async fn book_card(title:&str,author:&str,year:u32,finished:bool) -> Result {
view!{
    <article>
        <h2>(title)</h2>
        <p>
            (author)
            " - "
            (year)
        </p>
        <span
            class="badge"
            if finished {
                class="badge done"
            }
        >
            if finished {
                "FINISHED"
            } else {
                "Unread"
            }
        </span>
    </article>
}


}

