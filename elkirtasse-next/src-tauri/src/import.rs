use jetdb::Database;
use std::path::Path;
use anyhow::Result;
use crate::models::Page;

pub fn import_mdb(path: &Path) -> Result<(String, String, String, Vec<Page>)> {
    let mut db = Database::open(path)?;

    // Check if it's a .bok (Maktaba Shamela) file or a simple .mdb
    let tables = db.tables()?;

    let mut title = String::new();
    let mut author = String::new();
    let mut description = String::new();
    let mut pages = Vec::new();

    if tables.iter().any(|t| t.name == "Main") {
        // Maktaba Shamela .bok format
        let main_table = db.get_table("Main")?;
        let rows = main_table.decode_all_rows(None)?;
        if let Some(row) = rows.first() {
            title = row.get("Bk").and_then(|v| v.as_string()).unwrap_or_default();
            author = row.get("Auth").and_then(|v| v.as_string()).unwrap_or_default();
            description = row.get("Betaka").and_then(|v| v.as_string()).unwrap_or_default();
            let bk_id = row.get("BkId").and_then(|v| v.as_string()).unwrap_or_default();

            let book_table_name = format!("b{}", bk_id);
            if let Ok(book_table) = db.get_table(&book_table_name) {
                let book_rows = book_table.decode_all_rows(None)?;
                for row in book_rows {
                    pages.push(Page {
                        id: row.get("id").and_then(|v| v.as_string()).unwrap_or_default(),
                        nass: row.get("nass").and_then(|v| v.as_string()).unwrap_or_default(),
                        part: row.get("part").and_then(|v| v.as_string()).unwrap_or_default(),
                        page: row.get("page").and_then(|v| v.as_string()).unwrap_or_default(),
                    });
                }
            }
        }
    } else if tables.iter().any(|t| t.name == "book") {
        // Simple .mdb format
        title = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let book_table = db.get_table("book")?;
        let book_rows = book_table.decode_all_rows(None)?;
        for row in book_rows {
            pages.push(Page {
                id: row.get("id").and_then(|v| v.as_string()).unwrap_or_default(),
                nass: row.get("nass").and_then(|v| v.as_string()).unwrap_or_default(),
                part: row.get("part").and_then(|v| v.as_string()).unwrap_or_default(),
                page: row.get("page").and_then(|v| v.as_string()).unwrap_or_default(),
            });
        }
    }

    Ok((title, author, description, pages))
}
