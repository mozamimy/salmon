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

#[derive(Debug)]
pub struct Blog {
    src_dir: std::path::PathBuf,
    dest_dir: std::path::PathBuf,

    articles_by_tag: ArticlesByTag,
    pub sorted_articles: Vec<Rc<Article>>,
    layouts: Layouts,
    partials: Vec<Partial>,
    pages: Vec<Page>,
    resources: Vec<Resource>,
}

impl Blog {
    pub fn init(src_dir: PathBuf, dest_dir: PathBuf) -> Result<Self, Error> {
        let (articles_by_tag, sorted_articles) = load_articles(&src_dir)?;
        let layouts = load_layouts(&src_dir)?;
        let partials = load_partials(&src_dir)?;
        let pages = load_pages(&src_dir)?;
        let resources = load_resources(&src_dir)?;

        Ok(Blog {
            src_dir: src_dir,
            dest_dir: dest_dir,

            articles_by_tag: articles_by_tag,
            sorted_articles: sorted_articles,
            layouts: layouts,
            partials: partials,
            pages: pages,
            resources: resources,
        })
    }

    pub fn build(&self) -> Result<(), Error> {
        self.build_index_page()?;
        Ok(())
    }

    fn build_index_page(&self) -> Result<(), Error> {
        let mut renderer = Handlebars::new();
        renderer.register_escape_fn(handlebars::no_escape);

        let template_string = match &self.layouts.index {
            Layout::Index(s) => s,
            _ => return Err(format_err!("Invalid Layout variant.")),
        };
        for ref p in self.partials.iter() {
            renderer.register_partial(p.name.as_str(), p.template.as_str())?;
        }
        renderer.register_helper(
            "convert_to_iso8601",
            Box::new(view_helper::convert_to_iso8601),
        );

        std::fs::create_dir_all(self.dest_dir.join("page"))?;

        let paginator = Paginator::new(&self.sorted_articles, 10);
        let num_pages = paginator.len();
        for (mut i, page) in paginator.enumerate() {
            // The page number seen from users is 1 origin.
            i += 1;

            let mut data = Map::new();
            data.insert("articles".to_string(), handlebars::to_json(&page));

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
}
