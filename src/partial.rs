use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Partial {
    pub name: String,
    pub template: String,
}

pub fn load_partials(src_dir: &PathBuf) -> Result<Vec<Partial>, Error> {
    let mut partials = Vec::new();

    let partial_glob = glob::glob(src_dir.join("partials/**/*.hbs").to_str().unwrap())?;

    for entry in partial_glob {
        match entry {
            Ok(path) => partials.push(load_partial(&path)?),
            Err(e) => return Err(format_err!("{:?}", e)),
        }
    }

    Ok(partials)
}

fn load_partial(path: &PathBuf) -> Result<Partial, Error> {
    let mut file = File::open(path)?;
    let mut template = String::new();
    file.read_to_string(&mut template)?;

    Ok(Partial {
        name: path.file_stem().unwrap().to_string_lossy().to_string(),
        template: template,
    })
}
