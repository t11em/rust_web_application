use uuid::Uuid;

pub mod event;

#[derive(Debug)]
pub struct Book {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
