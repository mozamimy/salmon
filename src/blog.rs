use crate::article::ArticlesByTag;
use crate::article::*;
use crate::layout::Layout;
use crate::page::Page;
use crate::resource::Resource;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Blog {
    src_dir: std::path::PathBuf,
    dest_dir: std::path::PathBuf,

    articles_by_tag: ArticlesByTag,
    layouts: Vec<Layout>,
    pages: Vec<Page>,
    resources: Vec<Resource>,
}

impl Blog {
    pub fn init(src_dir: PathBuf, dest_dir: PathBuf) -> Self {
        let articles_by_tag = load_articles(&src_dir).unwrap();
        let layouts = scan_layouts();
        let pages = scan_pages();
        let resources = scan_resources();

        Blog {
            src_dir: src_dir,
            dest_dir: dest_dir,

            articles_by_tag: articles_by_tag,
            layouts: layouts,
            pages: pages,
            resources: resources,
        }
    }
}
fn scan_layouts() -> Vec<Layout> {
    Vec::new()
}
fn scan_pages() -> Vec<Page> {
    Vec::new()
}
fn scan_resources() -> Vec<Resource> {
    Vec::new()
}
