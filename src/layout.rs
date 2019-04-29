#[derive(Debug)]
pub enum Layout {
    Index(String),
    Article(String),
    Tag(String),
    Year(String),
    Page(String),
}
