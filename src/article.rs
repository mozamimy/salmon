use crate::converter;
use chrono::Datelike;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    title: String,
    pub date: chrono::NaiveDate,
    tags: Vec<String>,
    body: String,
    pub html: String,
    pub path: PathBuf,
}

pub type ArticlesByTag = HashMap<String, Vec<Rc<Article>>>;
pub type ArticlesByYear = HashMap<i32, Vec<Rc<Article>>>;

pub fn load_articles(
    src_dir: &PathBuf,
) -> Result<(ArticlesByTag, ArticlesByYear, Vec<Rc<Article>>), Error> {
    let mut articles_by_tag = ArticlesByTag::new();
    let mut articles_by_year = ArticlesByYear::new();
    let mut sorted_articles = Vec::new();

    let article_dir_glob = glob::glob(&src_dir.join("articles/**/*.md").to_str().unwrap())?;
    for entry in article_dir_glob {
        match entry {
            Ok(path) => {
                let article = Rc::new(load_article(src_dir, &path)?);
                for tag in article.tags.iter() {
                    if !articles_by_tag.contains_key(tag.as_str()) {
                        articles_by_tag.insert(tag.clone(), Vec::new());
                    }
                    articles_by_tag
                        .get_mut(tag.as_str())
                        .unwrap()
                        .push(article.clone());
                }
                let article_year = article.date.year();
                if !articles_by_year.contains_key(&article_year) {
                    articles_by_year.insert(article_year, Vec::new());
                }
                articles_by_year
                    .get_mut(&article_year)
                    .unwrap()
                    .push(article.clone());
                sorted_articles.push(article.clone());

                log::debug!("Article \"{}\" has been loaded.", &article.title);
            }
            Err(e) => return Err(failure::format_err!("{:?}", e)),
        }
    }
    sorted_articles.sort_by_key(|a| std::cmp::Reverse(a.date));
    for (_, articles) in articles_by_tag.iter_mut() {
        articles.sort_by_key(|a| std::cmp::Reverse(a.date));
    }
    for (_, articles) in articles_by_year.iter_mut() {
        articles.sort_by_key(|a| std::cmp::Reverse(a.date));
    }
    Ok((articles_by_tag, articles_by_year, sorted_articles))
}

fn load_article(src_dir: &PathBuf, article_path: &PathBuf) -> Result<Article, Error> {
    let mut file = File::open(article_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    let (title, date, tags, body) = decompose_source(&source)?;
    let html = converter::convert_to_html(&body);

    Ok(Article {
        title: title,
        date: date,
        tags: tags,
        body: body,
        html: html,
        path: PathBuf::from("/").join(
            article_path
                .strip_prefix(src_dir.join("articles/"))?
                .with_extension("html")
                .to_path_buf(),
        ),
    })
}

fn decompose_source(
    source: &str,
) -> Result<(String, chrono::NaiveDate, Vec<String>, String), Error> {
    let mut title = String::new();
    let mut date = chrono::NaiveDate::parse_from_str("2019-04-01", "%Y-%m-%d").unwrap();
    let mut tags = Vec::new();
    let mut body = String::with_capacity(source.len());

    let mut line_number = 0;
    for line in source.lines() {
        match line_number {
            0 | 4 => {
                if line.trim() != "---" {
                    return Err(failure::format_err!(
                        "Invalid markdown metadata format error.\nLine {} is not `---`",
                        line_number,
                    ));
                }
            }
            1..=3 => {
                let trimmed_line = line.trim();
                if trimmed_line.starts_with("title:") {
                    let v: Vec<&str> = trimmed_line.split(':').collect();
                    match v.get(1) {
                        Some(s) => title = s.trim().to_string(),
                        None => {
                            return Err(failure::format_err!(
                                "{} does not have any value",
                                v.get(0).unwrap()
                            ))
                        }
                    }
                } else if trimmed_line.starts_with("date:") {
                    let v: Vec<&str> = trimmed_line.split(':').collect();
                    match v.get(1) {
                        Some(s) => {
                            date = chrono::NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")?;
                        }
                        None => {
                            return Err(failure::format_err!(
                                "{} does not have any value",
                                v.get(0).unwrap()
                            ))
                        }
                    }
                } else if trimmed_line.starts_with("tags:") {
                    let v: Vec<&str> = trimmed_line.split(':').collect();
                    match v.get(1) {
                        Some(s) => {
                            for tag in s.trim().split(',') {
                                tags.push(tag.trim().to_string());
                            }
                        }
                        None => {
                            return Err(failure::format_err!(
                                "{} does not have any value",
                                v.get(0).unwrap()
                            ))
                        }
                    }
                } else {
                    return Err(failure::format_err!(
                        "Invalid markdown metadate element error.\n`{}` is not supported.",
                        line
                    ));
                }
            }
            _ => {
                body.push_str(line);
                body.push_str("\n");
            }
        }
        line_number += 1;
    }

    Ok((title, date, tags, body))
}
