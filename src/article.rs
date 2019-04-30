use crate::converter;
use failure::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
pub struct Article {
    title: String,
    pub date: chrono::Date<chrono::Utc>,
    tags: Vec<String>,
    body: String,
    html: String,
}

pub type ArticlesByTag = HashMap<String, Vec<Rc<Article>>>;

pub fn load_articles(src_dir: &PathBuf) -> Result<(ArticlesByTag, Vec<Rc<Article>>), Error> {
    let mut articles = ArticlesByTag::new();
    let mut sorted_articles = Vec::new();

    let article_dir_glob = glob::glob(&src_dir.join("articles/**/*.md").to_str().unwrap())?;

    for entry in article_dir_glob {
        println!("{:?}", entry);
        match entry {
            Ok(path) => {
                let article = Rc::new(load_article(&path)?);
                for tag in article.tags.iter() {
                    if !articles.contains_key(tag.as_str()) {
                        articles.insert(tag.clone(), Vec::new());
                    }
                    articles
                        .get_mut(tag.as_str())
                        .unwrap()
                        .push(article.clone());
                    sorted_articles.push(article.clone());
                }
            }
            Err(e) => return Err(format_err!("{:?}", e)),
        }
    }
    sorted_articles.sort_by_key(|a| a.date);
    Ok((articles, sorted_articles))
}

fn load_article(article_path: &PathBuf) -> Result<Article, Error> {
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
    })
}

fn decompose_source(
    source: &str,
) -> Result<(String, chrono::Date<chrono::Utc>, Vec<String>, String), Error> {
    let mut title = String::new();
    let mut date = chrono::Utc::today();
    let mut tags = Vec::new();
    let mut body = String::with_capacity(source.len());

    let mut line_number = 0;
    for line in source.lines() {
        match line_number {
            0 | 4 => {
                if line.trim() != "---" {
                    return Err(format_err!(
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
                            return Err(format_err!(
                                "{} does not have any value",
                                v.get(0).unwrap()
                            ))
                        }
                    }
                } else if trimmed_line.starts_with("date:") {
                    let v: Vec<&str> = trimmed_line.split(':').collect();
                    match v.get(1) {
                        Some(s) => {
                            let mut datetime_string = s.trim().to_string();
                            datetime_string.push_str(" 00:00:00 +00:00");
                            date = chrono::DateTime::parse_from_str(
                                datetime_string.as_str(),
                                "%Y-%m-%d %H:%M:%S %z",
                            )?
                            .with_timezone(&chrono::Utc)
                            .date();
                        }
                        None => {
                            return Err(format_err!(
                                "{} does not have any value",
                                v.get(0).unwrap()
                            ))
                        }
                    }
                } else if trimmed_line.starts_with("tags:") {
                    let v: Vec<&str> = trimmed_line.split(':').collect();
                    match v.get(1) {
                        Some(s) => {
                            for tag in s.trim().split_whitespace() {
                                tags.push(tag.to_string());
                            }
                        }
                        None => {
                            return Err(format_err!(
                                "{} does not have any value",
                                v.get(0).unwrap()
                            ))
                        }
                    }
                } else {
                    return Err(format_err!(
                        "Invalid markdown metadate element error.\n`{}` is not supported.",
                        line
                    ));
                }
            }
            _ => {
                body.push_str(line);
            }
        }
        line_number += 1;
    }

    Ok((title, date, tags, body))
}
