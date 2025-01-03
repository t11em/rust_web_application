pub mod book;
pub mod id;

#[derive(Debug)]
pub struct Book {
    pub book_id: id::BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
