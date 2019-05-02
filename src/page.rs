use crate::converter;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub path: PathBuf,
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

    let html = converter::convert_to_html(&body);

    Ok(Page {
        path: page_path
            .strip_prefix(src_dir.join("pages/"))?
            .to_path_buf(),
        body: body,
        html: html,
    })
}
