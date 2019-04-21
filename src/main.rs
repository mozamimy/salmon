extern crate clap;
#[macro_use]
extern crate failure;
extern crate glob;
extern crate pulldown_cmark;

mod builder;

fn main() {
    let matches = clap::App::new("salmon")
                    .version("0.1.0")
                    .author("mozamimy <alice@mozami.me>")
                    .about("A lightweight static site generator specialized for blogging.")
                    .subcommand(clap::SubCommand::with_name("build")
                        .arg(clap::Arg::with_name("SRC_DIR")
                            .help("Specify a directory which has salmon source files.")
                            .index(1)
                        )
                        .arg(clap::Arg::with_name("DEST_DIR")
                            .help("Specify a destination directory to put built files.")
                            .index(2)
                        )
                    ).get_matches();

    match matches {
        ref m if m.subcommand_matches("build").is_some() => {
            let src_path = std::path::Path::new(m.subcommand_matches("build").unwrap().value_of("SRC_DIR").unwrap_or("./"));
            let dest_path_buf = &src_path.join("build/");
            let dest_path;
            if let Some(dest_path_str) = m.subcommand_matches("build").unwrap().value_of("DEST_DIR") {
                dest_path = std::path::Path::new(dest_path_str);
            } else {
                dest_path = std::path::Path::new(dest_path_buf);
            }
            let builder = builder::Builder{
                src_path: src_path,
                dest_path: dest_path,
            };

            match builder.build() {
                Ok(_) => println!("Succeed to build."),
                Err(e) => { eprintln!("{:?}", e); std::process::exit(1) }
            }
        },
        _ => { eprintln!("Subcommand is not specified or unsupported subcommand.\nexit."); std::process::exit(1) },
    }
}
