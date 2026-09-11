#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Book{
    pub id:u32,
    pub title:String,
    pub author:String,
    pub year:u32,
    pub finished: bool,
}

pub fn all() -> Vec<Book> {
    vec![
        Book{id:1,title: "sefiller".to_string(), author: "Dost".to_string(), year: 2020, finished: true},
        Book{id:2,title: "reziller".to_string(), author: "hugo".to_string(), year: 2000, finished: true},
        Book{id:3,title: "filler".to_string(), author: "elt".to_string(), year: 2010, finished: false},
    ]
}

pub fn find(id:u32) -> Option<Book> {
    all().into_iter().find(|book| book.id == id)
}