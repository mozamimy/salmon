use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Layout {
    Index(String),
    Article(String),
    Tag(String),
    Year(String),
    Page(String),
    Rss(String),
}

#[derive(Debug)]
pub struct Layouts {
    pub index: Layout,
    pub article: Layout,
    pub tag: Layout,
    pub year: Layout,
    pub page: Layout,
    pub rss: Layout,
}

pub fn load_layouts(src_dir: &PathBuf) -> Result<Layouts, Error> {
    Ok(Layouts {
        index: load_index(src_dir)?,
        article: load_article(src_dir)?,
        tag: load_tag(src_dir)?,
        year: load_year(src_dir)?,
        page: load_page(src_dir)?,
        rss: load_rss(src_dir)?,
    })
}

fn load_index(src_dir: &PathBuf) -> Result<Layout, Error> {
    Ok(Layout::Index(load_file(src_dir, "index.hbs")?))
}

fn load_article(src_dir: &PathBuf) -> Result<Layout, Error> {
    Ok(Layout::Article(load_file(src_dir, "article.hbs")?))
}

fn load_tag(src_dir: &PathBuf) -> Result<Layout, Error> {
    Ok(Layout::Tag(load_file(src_dir, "tag.hbs")?))
}

fn load_year(src_dir: &PathBuf) -> Result<Layout, Error> {
    Ok(Layout::Year(load_file(src_dir, "year.hbs")?))
}

fn load_page(src_dir: &PathBuf) -> Result<Layout, Error> {
    Ok(Layout::Page(load_file(src_dir, "page.hbs")?))
}

fn load_rss(src_dir: &PathBuf) -> Result<Layout, Error> {
    Ok(Layout::Rss(load_file(src_dir, "rss.hbs")?))
}

fn load_file(src_dir: &PathBuf, file_name: &str) -> Result<String, Error> {
    let mut file = File::open(src_dir.join("layouts").join(file_name))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    log::debug!("Layout \"{}\" has been loaded.", file_name);
    Ok(content)
}
