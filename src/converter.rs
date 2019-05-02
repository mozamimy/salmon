use pulldown_cmark::html;
use pulldown_cmark::{Options, Parser};

pub fn convert_to_html(body: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(body, options);

    let mut built_html = String::with_capacity(body.len() * 3 / 2);
    html::push_html(&mut built_html, parser);

    built_html
}

pub fn highlight_code(content: &str, ext: &str) -> String {
    let ss = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let ts = syntect::highlighting::ThemeSet::load_defaults();
    let sr = ss.find_syntax_by_token(ext).unwrap();
    let theme = &ts.themes["Solarized (light)"];

    syntect::html::highlighted_html_for_string(content, &ss, &sr, &theme)
}
