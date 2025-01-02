use kernel::model::book::Book;
use uuid::Uuid;

pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        // この記法だと、必要のないフィールドを省略する際に便利
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        } = value;
        Book {
            book_id,
            title,
            author,
            isbn,
            description,
        }
    }
}
