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
                <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
            </head>
            <body class="bg-stone-50 text-stone-900 min-h-screen">
                <nav class="border-b border-stone-200 px-6 py-4 flex gab-6">
                    <a href="/" class="font-semibold">"Shelf"</a>
                    <span style="margin: 0 10px;">" "</span>
                    <a href="/about" class="text-stone-600">"About"</a>
                </nav>
                <main class="max-w-2xl mx-auto px-6 py-10">(slot?)</main>
            </body>
        </html>
    }
}