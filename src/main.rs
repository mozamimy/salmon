pub mod article;
pub mod blog;
pub mod code;
pub mod config;
pub mod converter;
pub mod initializer;
pub mod layout;
pub mod page;
pub mod paginator;
pub mod partial;
pub mod resource;
pub mod template_generator;
pub mod view_helper;

use crate::blog::Blog;
use crate::config::Config;
use crate::initializer::Initializer;

fn main() -> Result<(), failure::Error> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::try_init()?;

    let matches = clap::App::new("salmon")
        .version("0.5.0")
        .author("mozamimy <alice@mozami.me>")
        .about("A lightweight static site generator specialized for blogging")
        .subcommand(
            clap::SubCommand::with_name("build")
                .about("Build and generate articles")
                .arg(
                    clap::Arg::with_name("SRC_DIR")
                        .help("Specify a directory which has salmon source files")
                        .index(1),
                )
                .arg(
                    clap::Arg::with_name("DEST_DIR")
                        .help("Specify a destination directory to put built files")
                        .index(2),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("new")
                .about("Create new files and directories to start writing")
                .arg(
                    clap::Arg::with_name("ARTICLE_NAME")
                        .help("This value is used to determine article file name")
                        .index(1),
                )
                .arg(
                    clap::Arg::with_name("PROJECT_DIR")
                        .help("Specify your Salmon project directory")
                        .index(2),
                )
                .arg(
                    clap::Arg::with_name("article")
                        .short("a")
                        .long("article")
                        .help("Specify this if you want to create a template of article"),
                )
                .arg(
                    clap::Arg::with_name("code")
                        .short("c")
                        .long("code")
                        .help("Specify this if you want to create a directory for codes"),
                )
                .arg(
                    clap::Arg::with_name("image")
                        .short("i")
                        .long("image")
                        .help("Specify this if you want to create a directory for images"),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("init")
                .about("Initialize Salmon project")
                .arg(
                    clap::Arg::with_name("PROJECT_NAME")
                        .help("Your project name of Salmon")
                        .index(1),
                ),
        )
        .get_matches();

    match matches {
        ref m if m.subcommand_matches("build").is_some() => {
            let src_dir = std::path::PathBuf::from(
                m.subcommand_matches("build")
                    .unwrap()
                    .value_of("SRC_DIR")
                    .unwrap_or("./"),
            );

            let dest_dir;
            if let Some(dest_dir_str) = m.subcommand_matches("build").unwrap().value_of("DEST_DIR")
            {
                dest_dir = std::path::PathBuf::from(dest_dir_str);
            } else {
                dest_dir = std::path::PathBuf::from(&src_dir.join("build/"));
            }
            match std::fs::create_dir_all(&dest_dir) {
                Ok(_) => log::debug!("Created destination directory: {:?}", dest_dir),
                Err(e) => {
                    log::error!("Failed to create destination directory: {:?}", dest_dir);
                    log::error!("{:?}", e);
                    std::process::exit(1)
                }
            }

            let canonicalized_src_dir = src_dir.canonicalize().unwrap_or_else(|e| {
                log::error!(
                    "Failed to canonicalize source directory path: {:?}.",
                    src_dir
                );
                log::error!("{:?}", e);
                std::process::exit(1)
            });
            let canonicalized_dest_dir = dest_dir.canonicalize().unwrap_or_else(|e| {
                log::error!(
                    "Failed to canonicalize destination directory path: {:?}",
                    dest_dir
                );
                log::error!("{:?}", e);
                std::process::exit(1)
            });

            let config = Config::load(&canonicalized_src_dir)?;
            let init_blog_result =
                Blog::init(canonicalized_src_dir, canonicalized_dest_dir, config);
            match init_blog_result {
                Ok(blog) => blog.build().unwrap(),
                Err(e) => {
                    log::error!(
                        "An error is occured while loading components.\n{:?}\nexit.",
                        e
                    );
                    std::process::exit(1);
                }
            }
        }
        ref m if m.subcommand_matches("new").is_some() => {
            let project_dir = std::path::PathBuf::from(
                m.subcommand_matches("new")
                    .unwrap()
                    .value_of("PROJECT_DIR")
                    .unwrap_or("./"),
            );
            let article_name = m
                .subcommand_matches("new")
                .unwrap()
                .value_of("ARTICLE_NAME")
                .unwrap_or("no_title");
            match template_generator::generate_template(
                &m.subcommand_matches("new").unwrap(),
                &project_dir,
                &article_name,
            ) {
                Ok(_) => { /* do nothing */ }
                Err(e) => {
                    log::error!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        ref m if m.subcommand_matches("init").is_some() => {
            let project_dir = match std::env::current_dir() {
                Ok(d) => d.join(
                    m.subcommand_matches("init")
                        .unwrap()
                        .value_of("PROJECT_NAME")
                        .unwrap_or("salmon"),
                ),
                Err(e) => {
                    log::error!("Failed to get current directory.");
                    log::error!("{:?}", e);
                    std::process::exit(1);
                }
            };
            match std::fs::create_dir_all(&project_dir) {
                Ok(_) => {
                    let initializer = Initializer::new(project_dir);
                    let initialize_result = initializer.init();
                    match initialize_result {
                        Ok(_) => { /* do nothing */ }
                        Err(e) => {
                            log::error!("An error is occured while initializing project.");
                            log::error!("{:?}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to create project directory: {:?}", project_dir);
                    log::error!("{:?}", e);
                    std::process::exit(1)
                }
            };
        }
        _ => {
            log::error!("Subcommand is not specified or unsupported subcommand.\nexit.");
            std::process::exit(1)
        }
    }

    Ok(())
}
