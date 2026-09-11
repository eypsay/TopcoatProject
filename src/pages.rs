use topcoat::context::Cx;
use topcoat::router::{page, path_param};
use topcoat::router::error::RouterErrorExt;
use topcoat::view::view;
use crate::books;
use crate::component::book_card;

#[page("/")]
pub async fn home() -> topcoat::Result {
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
                id: book.id,
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
pub async  fn about()-> topcoat::Result {
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

path_param!(book_id:u32,error=not_found);
#[page("/books/{book_id}")]
pub async fn book_detail(cx: &Cx)-> topcoat::Result {
    let id=path_param::<BookId>(cx)?;
    let book =books::find(*id).ok_or_not_found()?;

    view!{
        <h1>(book.title.as_str())</h1>
        <p>(book.author.as_str())</p>
        if book.finished {
            <p>"You have Finished this book!"</p>
        } else {
            <p>"You're still reading this book!"</p>
        }
        <a href="/">"Back to the shlef"</a>
    }

}
