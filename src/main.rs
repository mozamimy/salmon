pub mod article;
pub mod blog;
pub mod code;
pub mod converter;
pub mod layout;
pub mod page;
pub mod paginator;
pub mod partial;
pub mod resource;
pub mod view_helper;

use crate::blog::Blog;

fn main() {
    let matches = clap::App::new("salmon")
        .version("0.1.0")
        .author("mozamimy <alice@mozami.me>")
        .about("A lightweight static site generator specialized for blogging.")
        .subcommand(
            clap::SubCommand::with_name("build")
                .arg(
                    clap::Arg::with_name("SRC_DIR")
                        .help("Specify a directory which has salmon source files.")
                        .index(1),
                )
                .arg(
                    clap::Arg::with_name("DEST_DIR")
                        .help("Specify a destination directory to put built files.")
                        .index(2),
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

            let canonicalized_src_dir = src_dir.canonicalize().unwrap_or_else(|e| {
                eprintln!("Failed to canonicalize source directory path: {:?}", e);
                std::process::exit(1)
            });
            let canonicalized_dest_dir = dest_dir.canonicalize().unwrap_or_else(|e| {
                eprintln!("Failed to canonicalize source directory path: {:?}", e);
                std::process::exit(1)
            });

            let init_blog_result = Blog::init(canonicalized_src_dir, canonicalized_dest_dir);
            match init_blog_result {
                Ok(blog) => blog.build().unwrap(),
                Err(e) => {
                    eprintln!(
                        "An error is occured while loading components.\n{:?}\nexit.",
                        e
                    );
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Subcommand is not specified or unsupported subcommand.\nexit.");
            std::process::exit(1)
        }
    }
}
