use crate::models::{Book, BookGroup, Page, TocEntry};
use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct XmlGroupRoot {
    #[serde(rename = "root", default)]
    groups: Vec<XmlGroup>,
}

#[derive(Debug, Deserialize)]
struct XmlGroup {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "Item", default)]
    items: Vec<XmlGroupItem>,
}

#[derive(Debug, Deserialize)]
struct XmlGroupItem {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "bk", default)]
    books: Vec<XmlBook>,
}

#[derive(Debug, Deserialize)]
struct XmlBook {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@aut")]
    author: String,
    #[serde(rename = "@betaka")]
    betaka: String,
    #[serde(rename = "@tfsr")]
    is_tefsir: Option<String>,
}

pub fn parse_library(xml_content: &str) -> Result<Vec<BookGroup>> {
    let root: XmlGroupRoot = from_str(xml_content)?;
    let mut groups = Vec::new();

    for xml_group in root.groups {
        for item in xml_group.items {
            let mut books = Vec::new();
            for xml_book in item.books {
                books.push(Book {
                    id: xml_book.id,
                    title: xml_book.name,
                    author: xml_book.author,
                    description: xml_book.betaka,
                    is_tefsir: xml_book.is_tefsir.unwrap_or_default() == "1",
                    group_id: item.id.clone(),
                });
            }
            groups.push(BookGroup {
                id: item.id,
                name: item.name,
                books,
            });
        }
    }

    Ok(groups)
}

#[derive(Debug, Deserialize)]
struct XmlBookRoot {
    #[serde(rename = "book", default)]
    pages: Vec<XmlPage>,
}

#[derive(Debug, Deserialize)]
struct XmlPage {
    id: String,
    nass: String,
    part: String,
    page: String,
}

pub fn parse_book(xml_content: &str) -> Result<Vec<Page>> {
    let root: XmlBookRoot = from_str(xml_content)?;
    Ok(root.pages.into_iter().map(|p| Page {
        id: p.id,
        nass: p.nass,
        part: p.part,
        page: p.page,
    }).collect())
}

#[derive(Debug, Deserialize)]
struct XmlTocRoot {
    #[serde(rename = "title", default)]
    titles: Vec<XmlTocEntry>,
}

#[derive(Debug, Deserialize)]
struct XmlTocEntry {
    id: String,
    tit: String,
    lvl: String,
}

pub fn parse_toc(xml_content: &str) -> Result<Vec<TocEntry>> {
    let root: XmlTocRoot = from_str(xml_content)?;
    Ok(root.titles.into_iter().map(|t| TocEntry {
        id: t.id,
        title: t.tit,
        level: t.lvl.parse().unwrap_or(1),
    }).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_library() {
        let xml = r#"<dataroot>
            <root Name="Main">
                <Item Name="Group1" id="1">
                    <bk id="bk1" name="Book1" aut="Author1" betaka="Info1" tfsr="0"/>
                </Item>
            </root>
        </dataroot>"#;
        let groups = parse_library(xml).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "Group1");
        assert_eq!(groups[0].books.len(), 1);
        assert_eq!(groups[0].books[0].title, "Book1");
    }

    #[test]
    fn test_parse_book() {
        let xml = r#"<dataroot>
            <book>
                <id>1</id>
                <nass>Content</nass>
                <part>1</part>
                <page>1</page>
            </book>
        </dataroot>"#;
        let pages = parse_book(xml).unwrap();
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].nass, "Content");
    }

    #[test]
    fn test_parse_toc() {
        let xml = r#"<dataroot>
            <title>
                <id>1</id>
                <tit>Chapter 1</tit>
                <lvl>1</lvl>
            </title>
        </dataroot>"#;
        let toc = parse_toc(xml).unwrap();
        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].title, "Chapter 1");
    }
}
