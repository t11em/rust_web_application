pub mod event;

use crate::model::{id::BookId, user::BookOwner};

#[derive(Debug)]
pub struct Book {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

#[derive(Debug)]
pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}
