extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Options, html};

fn main() {
    println!("Hello, world!");

    let markdown_input: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";

    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let parser = pulldown_cmark::Parser::new_ext(markdown_input, options);

    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    pulldown_cmark::html::push_html(&mut html_output, parser);

    println!("Out: {}", &html_output);
}
