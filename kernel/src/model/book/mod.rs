pub mod event;

use crate::model::id::BookId;

#[derive(Debug)]
pub struct Book {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
