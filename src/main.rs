extern crate clap;

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
                    ).get_matches();

    match matches {
        ref m if m.subcommand_matches("build").is_some() => {
            let builder: builder::Builder;
            if let Some(src_dir) = m.subcommand_matches("build").unwrap().value_of("SRC_DIR") {
                builder = builder::Builder{
                    root_path: std::path::Path::new(src_dir),
                }
            } else {
                builder = builder::Builder{
                    root_path: std::path::Path::new("./"),
                }
            }
            builder.build();
        },
        _ => { eprintln!("Subcommand is not specified or unsupported subcommand.\nexit."); std::process::exit(1) },
    }
}
