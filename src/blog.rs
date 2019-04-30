use crate::article::ArticlesByTag;
use crate::article::*;
use crate::layout::load_layouts;
use crate::layout::Layouts;
use crate::page::load_pages;
use crate::page::Page;
use crate::resource::load_resources;
use crate::resource::Resource;
use failure::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Blog {
    src_dir: std::path::PathBuf,
    dest_dir: std::path::PathBuf,

    articles_by_tag: ArticlesByTag,
    pub layouts: Layouts,
    pages: Vec<Page>,
    resources: Vec<Resource>,
}

impl Blog {
    pub fn init(src_dir: PathBuf, dest_dir: PathBuf) -> Result<Self, Error> {
        let articles_by_tag = load_articles(&src_dir)?;
        let layouts = load_layouts(&src_dir)?;
        let pages = load_pages(&src_dir)?;
        let resources = load_resources(&src_dir)?;

        Ok(Blog {
            src_dir: src_dir,
            dest_dir: dest_dir,

            articles_by_tag: articles_by_tag,
            layouts: layouts,
            pages: pages,
            resources: resources,
        })
    }
}
