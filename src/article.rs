use crate::tag::Tag;

pub struct Article {
    title: String,
    date: chrono::Date<chrono::Utc>,
    tags: Vec<Tag>,
}
