use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub is_tefsir: bool,
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookGroup {
    pub id: String,
    pub name: String,
    pub books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Library {
    pub groups: Vec<BookGroup>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    pub id: String,
    pub nass: String,
    pub part: String,
    pub page: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TocEntry {
    pub id: String,
    pub title: String,
    pub level: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub book_id: String,
    pub book_title: String,
    pub author: String,
    pub page_id: String,
    pub part: String,
    pub page: String,
    pub snippet: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Annotation {
    pub book_id: String,
    pub page_id: String,
    pub text_selection: String,
    pub note: String,
    pub color: String,
}
