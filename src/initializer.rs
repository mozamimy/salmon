use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Initializer {
    project_dir: PathBuf,
}

impl Initializer {
    pub fn new(project_dir: PathBuf) -> Self {
        Initializer { project_dir }
    }

    pub fn init(&self) -> Result<(), Error> {
        self.create_example_article()?;
        self.create_example_code()?;
        self.create_example_layouts()?;
        self.create_example_page()?;
        self.create_example_partials()?;
        self.create_example_resources()?;
        self.create_example_salmon_config()?;
        self.create_docker_compose_file()?;

        log::info!("Your new Salmon project has been initialized!");
        log::info!(
            "Now you can build with `salmon build` command after moved the project directory."
        );
        log::info!(
            "Execute `docker-compose up nginx` if you want to open your site in http://localhost:10080/."
        );

        Ok(())
    }

    fn create_example_article(&self) -> Result<(), Error> {
        log::info!("Writing /articles/2019/06/23/example.md");
        let article_dir = self
            .project_dir
            .join("articles")
            .join("2019")
            .join("06")
            .join("23");
        std::fs::create_dir_all(&article_dir)?;
        let article_path = article_dir.join("example.md");
        let mut article_file = File::create(article_path)?;
        article_file.write_all(include_bytes!("../example/articles/2019/06/23/example.md"))?;

        Ok(())
    }

    fn create_example_code(&self) -> Result<(), Error> {
        log::info!("Writing /codes/2019/06/23/example.rb");
        let code_dir = self
            .project_dir
            .join("codes")
            .join("2019")
            .join("06")
            .join("23");
        std::fs::create_dir_all(&code_dir)?;
        let code_path = code_dir.join("example.rb");
        let mut code_file = File::create(code_path)?;
        code_file.write_all(include_bytes!("../example/codes/2019/06/23/example.rb"))?;

        Ok(())
    }

    fn create_example_layouts(&self) -> Result<(), Error> {
        let layout_dir = self.project_dir.join("layouts");

        std::fs::create_dir_all(&layout_dir)?;

        log::info!("Writing /layouts/article.hbs");
        let mut article_file = File::create(layout_dir.join("article.hbs"))?;
        article_file.write_all(include_bytes!("../example/layouts/article.hbs"))?;
        log::info!("Writing /layouts/index.hbs");
        let mut index_file = File::create(layout_dir.join("index.hbs"))?;
        index_file.write_all(include_bytes!("../example/layouts/index.hbs"))?;
        log::info!("Writing /layouts/page.hbs");
        let mut page_file = File::create(layout_dir.join("page.hbs"))?;
        page_file.write_all(include_bytes!("../example/layouts/page.hbs"))?;
        log::info!("Writing /layouts/rss.hbs");
        let mut rss_file = File::create(layout_dir.join("rss.hbs"))?;
        rss_file.write_all(include_bytes!("../example/layouts/rss.hbs"))?;
        log::info!("Writing /layouts/tag.hbs");
        let mut tag_file = File::create(layout_dir.join("tag.hbs"))?;
        tag_file.write_all(include_bytes!("../example/layouts/tag.hbs"))?;
        log::info!("Writing /layouts/year.hbs");
        let mut year_file = File::create(layout_dir.join("year.hbs"))?;
        year_file.write_all(include_bytes!("../example/layouts/year.hbs"))?;

        Ok(())
    }

    fn create_example_page(&self) -> Result<(), Error> {
        log::info!("Writing /pages/example.md");
        let page_dir = self.project_dir.join("pages");
        std::fs::create_dir_all(&page_dir)?;
        let page_path = page_dir.join("example.md");
        let mut page_file = File::create(page_path)?;
        page_file.write_all(include_bytes!("../example/pages/example.md"))?;
        Ok(())
    }

    fn create_example_partials(&self) -> Result<(), Error> {
        let partial_dir = self.project_dir.join("partials");

        std::fs::create_dir_all(&partial_dir)?;

        log::info!("Writing /partials/header.hbs");
        let mut header_file = File::create(partial_dir.join("header.hbs"))?;
        header_file.write_all(include_bytes!("../example/partials/header.hbs"))?;
        log::info!("Writing /partials/menu.hbs");
        let mut menu_file = File::create(partial_dir.join("menu.hbs"))?;
        menu_file.write_all(include_bytes!("../example/partials/menu.hbs"))?;

        Ok(())
    }

    fn create_example_resources(&self) -> Result<(), Error> {
        let images_dir = self.project_dir.join("resources").join("images");
        std::fs::create_dir_all(&images_dir)?;
        log::info!("Writing /resources/images/sushi_salmon.png");
        let image_path = images_dir.join("sushi_salmon.png");
        let mut image_file = File::create(image_path)?;
        image_file.write_all(include_bytes!(
            "../example/resources/images/sushi_salmon.png"
        ))?;

        let stylesheet_dir = self.project_dir.join("resources").join("stylesheets");
        std::fs::create_dir_all(&stylesheet_dir)?;
        log::info!("Writing /resources/stylesheets/layout.sass");
        let stylesheet_path = stylesheet_dir.join("layout.sass");
        let mut stylesheet_file = File::create(stylesheet_path)?;
        stylesheet_file.write_all(include_bytes!(
            "../example/resources/stylesheets/layout.sass"
        ))?;

        Ok(())
    }

    fn create_example_salmon_config(&self) -> Result<(), Error> {
        log::info!("Writing /salmon.yaml");
        let mut config_file = File::create(self.project_dir.join("salmon.yaml"))?;
        config_file.write_all(include_bytes!("../example/salmon.yaml"))?;

        Ok(())
    }

    fn create_docker_compose_file(&self) -> Result<(), Error> {
        log::info!("Writing /docker-compose.yaml");
        let mut docker_compose_file = File::create(self.project_dir.join("docker-compose.yaml"))?;
        docker_compose_file.write_all(include_bytes!("../example/docker-compose.yaml"))?;

        Ok(())
    }
}
