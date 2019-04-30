use failure::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Resource {
    src_path: PathBuf,
    dest_path: PathBuf,
}

pub fn load_resources(src_dir: &PathBuf) -> Result<Vec<Resource>, Error> {
    let mut resources = Vec::new();

    let resource_glob = glob::glob(src_dir.join("resources/**/*.*").to_str().unwrap())?;

    for entry in resource_glob {
        match entry {
            Ok(path) => {
                let dest_path = path.strip_prefix(src_dir.join("resources/"))?;
                resources.push(Resource {
                    src_path: path.clone(),
                    dest_path: dest_path.to_path_buf(),
                })
            }
            Err(e) => return Err(format_err!("{:?}", e)),
        }
    }

    Ok(resources)
}
