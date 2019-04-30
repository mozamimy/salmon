use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Page {
    path: PathBuf,
    body: String,
    html: String,
}

pub fn load_pages(src_dir: &PathBuf) -> Result<Vec<Page>, Error> {
    let mut pages = Vec::new();

    let page_dir_glob = glob::glob(&src_dir.join("pages/**/*.md").to_str().unwrap())?;

    for entry in page_dir_glob {
        match entry {
            Ok(path) => pages.push(load_page(&path, src_dir)?),
            Err(e) => return Err(format_err!("{:?}", e)),
        }
    }

    Ok(pages)
}

fn load_page(page_path: &PathBuf, src_dir: &PathBuf) -> Result<Page, Error> {
    let mut file = File::open(page_path)?;
    let mut body = String::new();
    file.read_to_string(&mut body)?;

    let html = convert_to_html(&body);

    Ok(Page {
        path: page_path
            .strip_prefix(src_dir.join("pages/"))?
            .to_path_buf(),
        body: body,
        html: html,
    })
}

fn convert_to_html(body: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let parser = pulldown_cmark::Parser::new_ext(body, options);

    let mut built_html = String::with_capacity(body.len() * 3 / 2);
    pulldown_cmark::html::push_html(&mut built_html, parser);
    built_html
}
