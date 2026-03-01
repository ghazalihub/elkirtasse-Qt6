pub mod models;
pub mod parser;
pub mod import;
pub mod search;

use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use crate::models::{Book, BookGroup, Page, TocEntry, SearchResult, Annotation};
use crate::parser::{parse_library, parse_book, parse_toc};
use crate::search::SearchIndex;
use crate::import::import_mdb;
use std::sync::Mutex;

pub struct AppState {
    pub data_dir: PathBuf,
    pub search_index: Mutex<Option<SearchIndex>>,
}

#[tauri::command]
pub fn get_library(state: tauri::State<AppState>) -> Result<Vec<BookGroup>, String> {
    let group_xml_path = state.data_dir.join("data/group.xml");
    let content = std::fs::read_to_string(group_xml_path).map_err(|e| e.to_string())?;
    parse_library(&content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_book_content(book_id: String, state: tauri::State<AppState>) -> Result<Vec<Page>, String> {
    // Try custom books first then bundled books
    let book_path = state.data_dir.join(format!("{}/book.xml", book_id));
    let content = if book_path.exists() {
        std::fs::read_to_string(book_path).map_err(|e| e.to_string())?
    } else {
        // Fallback or error
        return Err("Book not found".to_string());
    };
    parse_book(&content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_book_toc(book_id: String, state: tauri::State<AppState>) -> Result<Vec<TocEntry>, String> {
    let toc_path = state.data_dir.join(format!("{}/title.xml", book_id));
    let content = if toc_path.exists() {
        std::fs::read_to_string(toc_path).map_err(|e| e.to_string())?
    } else {
        return Err("TOC not found".to_string());
    };
    parse_toc(&content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_books(query: String, state: tauri::State<'_, AppState>) -> Result<Vec<SearchResult>, String> {
    let index_lock = state.search_index.lock().map_err(|e| e.to_string())?;
    if let Some(index) = &*index_lock {
        index.search(&query, 50).map_err(|e| e.to_string())
    } else {
        Err("Search index not initialized".to_string())
    }
}

#[tauri::command]
pub async fn import_book_mdb_command(path: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let (title, author, description, pages) = import_mdb(std::path::Path::new(&path)).map_err(|e| e.to_string())?;

    let book_id = format!("bk_custom_{}", title.chars().filter(|c| c.is_alphanumeric()).collect::<String>());
    let book_dir = state.data_dir.join(&book_id);
    std::fs::create_dir_all(&book_dir).map_err(|e| e.to_string())?;

    // Save pages as book.xml
    let mut book_xml = String::from("<dataroot>\n");
    for page in &pages {
        book_xml.push_str(&format!(
            "  <book>\n    <id>{}</id>\n    <nass>{}</nass>\n    <part>{}</part>\n    <page>{}</page>\n  </book>\n",
            page.id, page.nass, page.part, page.page
        ));
    }
    book_xml.push_str("</dataroot>");
    std::fs::write(book_dir.join("book.xml"), book_xml).map_err(|e| e.to_string())?;

    // Index the book
    let index_lock = state.search_index.lock().map_err(|e| e.to_string())?;
    if let Some(index) = &*index_lock {
        index.add_pages(&book_id, &title, &author, &pages).map_err(|e| e.to_string())?;
    }

    Ok(book_id)
}

#[tauri::command]
pub fn save_annotation(annotation: Annotation, state: tauri::State<AppState>) -> Result<(), String> {
    let annotations_path = state.data_dir.join("annotations.json");
    let mut annotations: Vec<Annotation> = if annotations_path.exists() {
        let content = std::fs::read_to_string(&annotations_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    annotations.push(annotation);
    let content = serde_json::to_string(&annotations).map_err(|e| e.to_string())?;
    std::fs::write(annotations_path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_annotations(book_id: String, state: tauri::State<AppState>) -> Result<Vec<Annotation>, String> {
    let annotations_path = state.data_dir.join("annotations.json");
    if !annotations_path.exists() {
        return Ok(Vec::new());
    }
    let content = std::fs::read_to_string(&annotations_path).map_err(|e| e.to_string())?;
    let annotations: Vec<Annotation> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(annotations.into_iter().filter(|a| a.book_id == book_id).collect())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_dir = app.path().home_dir()
                .map_err(|e| tauri::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string())))?
                .join(".kirtasse");
            let index_path = data_dir.join("search_index");
            let search_index = SearchIndex::open_or_create(&index_path).ok();

            app.manage(AppState {
                data_dir,
                search_index: Mutex::new(search_index),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_library,
            get_book_content,
            get_book_toc,
            search_books,
            import_book_mdb_command,
            save_annotation,
            get_annotations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
