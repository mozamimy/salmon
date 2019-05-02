use crate::article::ArticlesByTag;
use crate::article::*;
use crate::layout::load_layouts;
use crate::layout::{Layout, Layouts};
use crate::page::load_pages;
use crate::page::Page;
use crate::paginator::Paginator;
use crate::partial::load_partials;
use crate::partial::Partial;
use crate::resource::load_resources;
use crate::resource::Resource;
use crate::view_helper;
use failure::Error;
use handlebars::Handlebars;
use serde_json::value::Map;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

type ViewItems = std::vec::Vec<serde_json::Map<std::string::String, handlebars::JsonValue>>;

#[derive(Debug)]
pub struct Blog {
    src_dir: std::path::PathBuf,
    dest_dir: std::path::PathBuf,

    articles_by_tag: ArticlesByTag,
    articles_by_year: ArticlesByYear,
    sorted_articles: Vec<Rc<Article>>,
    layouts: Layouts,
    partials: Vec<Partial>,
    pages: Vec<Page>,
    pub resources: Vec<Resource>,
}

impl Blog {
    pub fn init(src_dir: PathBuf, dest_dir: PathBuf) -> Result<Self, Error> {
        let (articles_by_tag, articles_by_year, sorted_articles) = load_articles(&src_dir)?;
        let layouts = load_layouts(&src_dir)?;
        let partials = load_partials(&src_dir)?;
        let pages = load_pages(&src_dir)?;
        let resources = load_resources(&src_dir)?;

        Ok(Blog {
            src_dir: src_dir,
            dest_dir: dest_dir,

            articles_by_tag: articles_by_tag,
            articles_by_year: articles_by_year,
            sorted_articles: sorted_articles,
            layouts: layouts,
            partials: partials,
            pages: pages,
            resources: resources,
        })
    }

    pub fn build(&self) -> Result<(), Error> {
        let renderer = self.init_renderer()?;
        let tags = self.init_tags();
        let years = self.init_years();
        let recent_articles = self.init_recent_articles();
        self.build_index_page(&renderer, &tags, &years, &recent_articles)?;
        self.build_article_page(&renderer, &tags, &years, &recent_articles)?;
        self.build_tag_page(&renderer, &tags, &years, &recent_articles)?;
        self.build_year_page(&renderer, &tags, &years, &recent_articles)?;
        self.build_general_page(&renderer)?;
        self.put_resources()?;
        Ok(())
    }

    fn build_index_page(
        &self,
        renderer: &Handlebars,
        tags: &ViewItems,
        years: &ViewItems,
        recent_articles: &[Rc<Article>],
    ) -> Result<(), Error> {
        let template_string = match &self.layouts.index {
            Layout::Index(s) => s,
            _ => return Err(format_err!("Invalid Layout variant.")),
        };

        std::fs::create_dir_all(self.dest_dir.join("page"))?;

        let paginator = Paginator::new(&self.sorted_articles, 10);
        let num_pages = paginator.len();
        for (mut i, page) in paginator.enumerate() {
            // The page number seen from users is 1 origin.
            i += 1;

            let mut data = Map::new();
            data.insert("articles".to_string(), handlebars::to_json(&page));
            data.insert("tags".to_string(), handlebars::to_json(tags));
            data.insert("years".to_string(), handlebars::to_json(years));
            data.insert(
                "recent_articles".to_string(),
                handlebars::to_json(recent_articles),
            );

            let mut paginate = Map::new();
            paginate.insert("page_number".to_string(), json!(i));
            paginate.insert("num_pages".to_string(), json!(num_pages));
            if i > 1 {
                paginate.insert("prev_page".to_string(), json!(format!("/page/{}/", i - 1)));
            }
            if i < num_pages {
                paginate.insert("next_page".to_string(), json!(format!("/page/{}/", i + 1)));
            }
            data.insert("paginate".to_string(), handlebars::to_json(&paginate));

            let html = renderer.render_template(template_string.as_str(), &data)?;
            let dest_path = if i == 1 {
                self.dest_dir.join("index.html")
            } else {
                let dest_file_dir = self.dest_dir.join("page").join(&i.to_string());
                std::fs::create_dir_all(&dest_file_dir)?;
                dest_file_dir.join("index.html")
            };
            let mut file = File::create(dest_path)?;
            file.write_all(html.as_bytes())?;
        }

        Ok(())
    }

    fn build_article_page(
        &self,
        renderer: &Handlebars,
        tags: &ViewItems,
        years: &ViewItems,
        recent_articles: &[Rc<Article>],
    ) -> Result<(), Error> {
        let template_string = match &self.layouts.article {
            Layout::Article(s) => s,
            _ => return Err(format_err!("Invalid Layout variant.")),
        };

        for article in self.sorted_articles.iter() {
            let mut data = Map::new();
            data.insert("article".to_string(), handlebars::to_json(&article));
            data.insert("tags".to_string(), handlebars::to_json(tags));
            data.insert("years".to_string(), handlebars::to_json(years));
            data.insert(
                "recent_articles".to_string(),
                handlebars::to_json(recent_articles),
            );

            let html = renderer.render_template(template_string.as_str(), &data)?;
            let dest_full_path = self.dest_dir.join(&article.path.strip_prefix("/")?);
            std::fs::create_dir_all(self.extract_parent_dir(&dest_full_path)?)?;
            let mut file = File::create(dest_full_path)?;
            file.write_all(html.as_bytes())?;
        }

        Ok(())
    }

    fn build_tag_page(
        &self,
        renderer: &Handlebars,
        tags: &ViewItems,
        years: &ViewItems,
        recent_articles: &[Rc<Article>],
    ) -> Result<(), Error> {
        let template_string = match &self.layouts.tag {
            Layout::Tag(s) => s,
            _ => return Err(format_err!("Invalid Layout variant.")),
        };

        for (tag, articles) in self.articles_by_tag.iter() {
            let mut data = Map::new();
            data.insert("tags".to_string(), handlebars::to_json(tags));
            data.insert("years".to_string(), handlebars::to_json(years));
            data.insert("tag_name".to_string(), handlebars::to_json(&tag));
            data.insert(
                "recent_articles".to_string(),
                handlebars::to_json(recent_articles),
            );

            let paginator = Paginator::new(&articles, 15);
            let num_pages = paginator.len();
            for (mut i, page) in paginator.enumerate() {
                // The page number seen from users is 1 origin.
                i += 1;

                data.insert("articles".to_string(), handlebars::to_json(page));

                let mut paginate = Map::new();
                paginate.insert("page_number".to_string(), json!(i));
                paginate.insert("num_pages".to_string(), json!(num_pages));
                if i > 1 {
                    if i == 2 {
                        paginate.insert(
                            "prev_page".to_string(),
                            json!(format!("/tags/{}.html", tag)),
                        );
                    } else {
                        paginate.insert(
                            "prev_page".to_string(),
                            json!(format!("/tags/{}/page/{}.html", tag, i - 1)),
                        );
                    }
                }
                if i < num_pages {
                    paginate.insert(
                        "next_page".to_string(),
                        json!(format!("/tags/{}/page/{}.html", tag, i + 1)),
                    );
                }
                data.insert("paginate".to_string(), handlebars::to_json(&paginate));

                let html = renderer.render_template(template_string.as_str(), &data)?;
                let dest_full_path = if i == 1 {
                    self.dest_dir.join("tags").join(&tag).with_extension("html")
                } else {
                    self.dest_dir
                        .join("tags")
                        .join(&tag)
                        .join("page")
                        .join(&i.to_string())
                        .with_extension("html")
                };
                std::fs::create_dir_all(self.extract_parent_dir(&dest_full_path)?)?;
                let mut file = File::create(dest_full_path)?;
                file.write_all(html.as_bytes())?;
            }
        }

        Ok(())
    }

    fn build_year_page(
        &self,
        renderer: &Handlebars,
        tags: &ViewItems,
        years: &ViewItems,
        recent_articles: &[Rc<Article>],
    ) -> Result<(), Error> {
        let template_string = match &self.layouts.year {
            Layout::Year(s) => s,
            _ => return Err(format_err!("Invalid Layout variant.")),
        };

        for (year, articles) in self.articles_by_year.iter() {
            let mut data = Map::new();
            data.insert("tags".to_string(), handlebars::to_json(tags));
            data.insert("years".to_string(), handlebars::to_json(years));
            data.insert("year_num".to_string(), handlebars::to_json(year));
            data.insert(
                "recent_articles".to_string(),
                handlebars::to_json(recent_articles),
            );
            let paginator = Paginator::new(&articles, 15);
            let num_pages = paginator.len();
            for (mut i, page) in paginator.enumerate() {
                // The page number seen from users is 1 origin.
                i += 1;

                data.insert("articles".to_string(), handlebars::to_json(page));

                let mut paginate = Map::new();
                paginate.insert("page_number".to_string(), json!(i));
                paginate.insert("num_pages".to_string(), json!(num_pages));
                if i > 1 {
                    if i == 2 {
                        paginate.insert("prev_page".to_string(), json!(format!("/{}.html", year)));
                    } else {
                        paginate.insert(
                            "prev_page".to_string(),
                            json!(format!("/{}/page/{}.html", year, i - 1)),
                        );
                    }
                }
                if i < num_pages {
                    paginate.insert(
                        "next_page".to_string(),
                        json!(format!("/{}/page/{}.html", year, i + 1)),
                    );
                }
                data.insert("paginate".to_string(), handlebars::to_json(&paginate));

                let html = renderer.render_template(template_string.as_str(), &data)?;
                let dest_full_path = if i == 1 {
                    self.dest_dir.join(&year.to_string()).with_extension("html")
                } else {
                    self.dest_dir
                        .join(&year.to_string())
                        .join("page")
                        .join(&i.to_string())
                        .with_extension("html")
                };
                std::fs::create_dir_all(self.extract_parent_dir(&dest_full_path)?)?;
                let mut file = File::create(dest_full_path)?;
                file.write_all(html.as_bytes())?;
            }
        }
        Ok(())
    }

    fn build_general_page(&self, renderer: &Handlebars) -> Result<(), Error> {
        let template_string = match &self.layouts.page {
            Layout::Page(s) => s,
            _ => return Err(format_err!("Invalid Layout variant.")),
        };

        for page in self.pages.iter() {
            let mut data = Map::new();
            data.insert("page".to_string(), handlebars::to_json(page));

            let html = renderer.render_template(template_string.as_str(), &data)?;
            let dest_full_path = self.dest_dir.join(&page.path).with_extension("html");
            std::fs::create_dir_all(self.extract_parent_dir(&dest_full_path)?)?;
            let mut file = File::create(dest_full_path)?;
            file.write_all(html.as_bytes())?;
        }

        Ok(())
    }

    fn put_resources(&self) -> Result<(), Error> {
        for resource in self.resources.iter() {
            match resource {
                Resource::StyleSheet(r) => {
                    let dest_full_path = self.dest_dir.join(&r.dest_path);
                    std::fs::create_dir_all(self.extract_parent_dir(&dest_full_path)?)?;
                    let mut file = File::create(dest_full_path)?;
                    file.write_all(r.compiled.as_bytes())?;
                }
                Resource::General(r) => {
                    let dest_full_path = self.dest_dir.join(&r.dest_path);
                    std::fs::create_dir_all(self.extract_parent_dir(&dest_full_path)?)?;
                    std::fs::copy(&r.src_path, dest_full_path)?;
                }
            }
        }

        Ok(())
    }

    fn init_renderer(&self) -> Result<Handlebars, Error> {
        let mut renderer = Handlebars::new();
        renderer.register_escape_fn(handlebars::no_escape);

        for ref p in self.partials.iter() {
            renderer.register_partial(p.name.as_str(), p.template.as_str())?;
        }

        renderer.register_helper(
            "convert_to_iso8601",
            Box::new(view_helper::convert_to_iso8601),
        );
        renderer.register_helper(
            "article_ogp_meta_tags",
            Box::new(view_helper::article_ogp_meta_tags),
        );

        Ok(renderer)
    }

    fn init_tags(&self) -> ViewItems {
        let mut tag_keys: Vec<_> = self.articles_by_tag.keys().collect();
        tag_keys.sort();
        let mut tags = Vec::new();
        for tag_key in tag_keys {
            let mut m = Map::new();
            m.insert("key".to_string(), json!(tag_key));
            m.insert(
                "len".to_string(),
                json!(self.articles_by_tag.get(tag_key).unwrap().len()),
            );
            tags.push(m);
        }
        tags
    }

    fn init_years(&self) -> ViewItems {
        let mut years = Vec::new();
        for (year, articles) in &self.articles_by_year {
            let mut m = Map::new();
            m.insert("year".to_string(), json!(year));
            m.insert("len".to_string(), json!(articles.len()));
            years.push(m);
        }
        years.sort_by(|v, u| {
            if v.get("year").unwrap().as_u64().unwrap() > u.get("year").unwrap().as_u64().unwrap() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        years
    }

    fn init_recent_articles(&self) -> &[Rc<Article>] {
        &self.sorted_articles[0..=10]
    }

    fn extract_parent_dir(&self, dest_full_path: &PathBuf) -> Result<PathBuf, Error> {
        Ok(dest_full_path
            .parent()
            .ok_or(format_err!("Directory not found"))?
            .to_path_buf())
    }
}
