use topcoat::router::layout;
use topcoat::view::view;

#[layout("/")]
pub async fn root_layout(slot: topcoat::Result) -> topcoat::Result {
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