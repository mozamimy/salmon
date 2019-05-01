use failure::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Resource {
    StyleSheet(Sass),
    General(GeneralResource),
}

#[derive(Debug)]
pub struct GeneralResource {
    pub src_path: PathBuf,
    pub dest_path: PathBuf,
}

#[derive(Debug)]
pub struct Sass {
    pub src_path: PathBuf,
    pub dest_path: PathBuf,
    pub compiled: String,
}

pub fn load_resources(src_dir: &PathBuf) -> Result<Vec<Resource>, Error> {
    let mut resources = Vec::new();

    let resource_glob = glob::glob(src_dir.join("resources/**/*.*").to_str().unwrap())?;

    for entry in resource_glob {
        match entry {
            Ok(path) => {
                let resource: Resource;
                let dest_path = path.strip_prefix(src_dir.join("resources/"))?.to_path_buf();
                match path.extension() {
                    Some(ex) if ex == "sass" => {
                        resource = Resource::StyleSheet(load_sass(&path, &dest_path)?);
                    }
                    Some(_) => {
                        resource = Resource::General(GeneralResource {
                            src_path: path.clone(),
                            dest_path: dest_path,
                        })
                    }
                    None => unreachable!(),
                }
                resources.push(resource);
            }
            Err(e) => return Err(format_err!("{:?}", e)),
        }
    }

    Ok(resources)
}

fn load_sass(src_path: &PathBuf, dest_path: &PathBuf) -> Result<Sass, Error> {
    let compiled = sass_rs::compile_file(src_path, sass_rs::Options::default())
        .map_err(|e| format_err!("{:?}", e))?;

    Ok(Sass {
        src_path: src_path.clone(),
        dest_path: dest_path.with_extension("css"),
        compiled: compiled,
    })
}
