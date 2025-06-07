#[derive(Debug)]
pub struct PaginatedList<T> {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub items: Vec<T>,
}

impl<T> PaginatedList {
    pub fn into_inner(self) -> Vec<T> {
        self.items
    }
}
