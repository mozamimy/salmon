use failure::Error;
use pulldown_cmark::html;
use pulldown_cmark::{Options, Parser};
use std::path::PathBuf;

pub fn convert_to_html(body: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(body, options);

    let mut built_html = String::with_capacity(body.len() * 3 / 2);
    html::push_html(&mut built_html, parser);

    built_html
}

pub fn highlight_code(
    content: &str,
    ext: Option<&String>,
    code_path: &PathBuf,
) -> Result<String, Error> {
    let ss = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let ts = syntect::highlighting::ThemeSet::load_defaults();
    let theme = &ts.themes["Solarized (light)"];
    let sr = match ss.find_syntax_by_first_line(content) {
        Some(s) => s,
        None => {
            if ext.is_none() {
                log::warn!(
                    "Cannot determine syntax for {:?}. Falling back to plain text mode.",
                    code_path
                );
                ss.find_syntax_plain_text()
            } else {
                match ss.find_syntax_by_token(ext.unwrap()) {
                    Some(s) => s,
                    None => {
                        log::warn!(
                            "Cannot determine syntax for {:?}. Falling back to plain text mode.",
                            code_path
                        );
                        ss.find_syntax_plain_text()
                    }
                }
            }
        }
    };

    Ok(syntect::html::highlighted_html_for_string(
        content, &ss, &sr, &theme,
    ))
}
