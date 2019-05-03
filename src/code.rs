use crate::converter;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    ext: Option<String>,
    content: String,
    highlighted_html: String,
}

pub fn load_codes(src_dir: &PathBuf) -> Result<HashMap<PathBuf, Code>, Error> {
    let mut codes = HashMap::new();

    let code_dir_glob = glob::glob(&src_dir.join("codes/**/*").to_str().unwrap())?;

    for entry in code_dir_glob {
        match entry {
            Ok(path) => {
                if std::fs::metadata(&path)?.is_file() {
                    let code = load_code(&path)?;
                    let key_path = PathBuf::from("/")
                        .join(path.strip_prefix(src_dir.join("codes/"))?.to_path_buf());
                    codes.insert(key_path, code);
                }
            }
            Err(e) => return Err(format_err!("{:?}", e)),
        }
    }

    Ok(codes)
}

fn load_code(code_path: &PathBuf) -> Result<Code, Error> {
    let ext = code_path
        .extension()
        .and_then(|e| Some(e.to_string_lossy().to_string()));

    let mut file = File::open(code_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let highlighted_html = converter::highlight_code(&content, ext.as_ref(), code_path)?;

    Ok(Code {
        ext: ext,
        content: content,
        highlighted_html: highlighted_html,
    })
}
