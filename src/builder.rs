use std::io::prelude::*;
use crate::article::Article;
use crate::tag::Tag;

pub struct Builder<'a> {
    src_path: &'a std::path::Path,
    dest_path: &'a std::path::Path,
    articles: Vec<Article>,
    tags: Vec<Tag>,
}

pub fn new<'a>(src_path: &'a std::path::Path, dest_path: &'a std::path::Path) -> Builder<'a> {
    Builder {
        src_path: src_path,
        dest_path: dest_path,
        articles: Vec::new(),
        tags: Vec::new(),
    }
}

impl<'a> Builder<'a> {
    pub fn build(&self) -> Result<(), failure::Error> {
        let article_path_buf = self.src_path.join("articles/**/*.md");
        let article_md_glob = article_path_buf.to_str().unwrap();

        for entry in glob::glob(article_md_glob)? {
            match entry {
                Ok(path) => self.build_md_file(&path)?,
                Err(e) => return Err(format_err!("{:?}", e)),
            }
        }

        Ok(())
    }

    fn build_md_file(&self, path: &std::path::PathBuf) -> Result<(), failure::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut source_md = String::new();
        file.read_to_string(&mut source_md)?;

        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        let parser = pulldown_cmark::Parser::new_ext(&source_md, options);

        let mut built_html = String::with_capacity(source_md.len() * 3 / 2);
        pulldown_cmark::html::push_html(&mut built_html, parser);

        self.write_built_file(path, &built_html)?;

        Ok(())
    }

    fn write_built_file(&self, source_path: &std::path::PathBuf, html: &str) -> Result<(), failure::Error> {
        let path_without_src_prefix = source_path.strip_prefix(self.src_path.join("articles/"))?;
        let html_path = self.dest_path.join(path_without_src_prefix).with_extension("html");

        std::fs::create_dir_all(html_path.parent().unwrap())?;
        let mut file = std::fs::File::create(&html_path)?;
        file.write_all(html.as_bytes())?;

        println!("Wrote: {}", html_path.display());

        Ok(())
    }
}
