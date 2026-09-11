use topcoat::view::{component, view};

#[component]
pub async fn book_card(id:u32 , title:&str,author:&str,year:u32,finished:bool) -> topcoat::Result {
    view!{
        <article>
            //<h2>(title)</h2>
            <h2><a href=(format!("/books/{id}"))>(title)</a></h2>
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
