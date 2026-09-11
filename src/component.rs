use topcoat::view::{component, view};

#[component]
pub async fn book_card(id:u32 , title:&str,author:&str,year:u32,finished:bool) -> topcoat::Result {
    view!{
        <article class="border border-stone-200 rounded-lg p-4 mb-3 bg-red">
            //<h2>(title)</h2>
            <h2><a href=(format!("/books/{id}"))>(title)</a></h2>
            <p class="text-sm text-stone-900 mt-1">
                (author)
                " - "
                (year)
            </p>
            <span
                class="inline-block mt-3 text-xs px-2 py-1 rounded bg-stone-100 text-stone-600"
                if finished {
                    class="inline-block mt-3 text-xs px-2 py-1 rounded bg-emerald-100 text-emerald-800"
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
