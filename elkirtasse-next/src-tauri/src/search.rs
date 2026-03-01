use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy, IndexReader};
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use std::path::Path;
use anyhow::Result;
use crate::models::{Page, SearchResult};

pub struct SearchIndex {
    index: Index,
    reader: IndexReader,
    schema: Schema,
}

impl SearchIndex {
    pub fn open_or_create(path: &Path) -> Result<Self> {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("book_id", STRING | STORED);
        schema_builder.add_text_field("book_title", TEXT | STORED);
        schema_builder.add_text_field("author", TEXT | STORED);
        schema_builder.add_text_field("page_id", STRING | STORED);
        schema_builder.add_text_field("part", STRING | STORED);
        schema_builder.add_text_field("page", STRING | STORED);
        schema_builder.add_text_field("content", TEXT | STORED);
        let schema = schema_builder.build();

        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }

        let index = Index::open_or_create(tantivy::directory::MmapDirectory::open(path)?, schema.clone())?;
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        Ok(Self { index, reader, schema })
    }

    pub fn add_pages(&self, book_id: &str, book_title: &str, author: &str, pages: &[Page]) -> Result<()> {
        let mut index_writer: IndexWriter = self.index.writer(50_000_000)?;
        let book_id_field = self.schema.get_field("book_id").unwrap();
        let book_title_field = self.schema.get_field("book_title").unwrap();
        let author_field = self.schema.get_field("author").unwrap();
        let page_id_field = self.schema.get_field("page_id").unwrap();
        let part_field = self.schema.get_field("part").unwrap();
        let page_field = self.schema.get_field("page").unwrap();
        let content_field = self.schema.get_field("content").unwrap();

        for page in pages {
            index_writer.add_document(tantivy::doc!(
                book_id_field => book_id,
                book_title_field => book_title,
                author_field => author,
                page_id_field => page.id.as_str(),
                part_field => page.part.as_str(),
                page_field => page.page.as_str(),
                content_field => page.nass.as_str()
            ))?;
        }

        index_writer.commit()?;
        Ok(())
    }

    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let searcher = self.reader.searcher();
        let content_field = self.schema.get_field("content").unwrap();
        let query_parser = QueryParser::for_index(&self.index, vec![content_field]);
        let query = query_parser.parse_query(query_str)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
        let mut results = Vec::new();

        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

            results.push(SearchResult {
                book_id: retrieved_doc.get_first(self.schema.get_field("book_id").unwrap()).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                book_title: retrieved_doc.get_first(self.schema.get_field("book_title").unwrap()).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                author: retrieved_doc.get_first(self.schema.get_field("author").unwrap()).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                page_id: retrieved_doc.get_first(self.schema.get_field("page_id").unwrap()).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                part: retrieved_doc.get_first(self.schema.get_field("part").unwrap()).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                page: retrieved_doc.get_first(self.schema.get_field("page").unwrap()).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                snippet: retrieved_doc.get_first(self.schema.get_field("content").unwrap()).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            });
        }

        Ok(results)
    }
}
