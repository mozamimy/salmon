use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use scraper::{Html, Selector};

pub fn convert_to_iso8601(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let date_str = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderError::new(
            "convert_to_iso8601: Param 0 with string type is required.",
        ))?;

    if let Ok(date_iso8601) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        // NativeDate doesn't have timezone and time information. We should fill it with some value.
        out.write(
            date_iso8601
                .format("%Y-%m-%dT00:00:00+00:00")
                .to_string()
                .as_str(),
        )?;
        Ok(())
    } else {
        Err(RenderError::new(format!(
            "Parsing date error with `{:?}`",
            date_str
        )))
    }
}

// TODO: We should give up parsing HTML directly. That is performance killer.
//       We can collect metadata from articles in article module when parsing source markdown files.
pub fn article_ogp_meta_tags(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let article = h
        .param(0)
        .and_then(|v| v.value().as_object())
        .ok_or(RenderError::new(
            "article_meta_tags: Param 0 with JSON object type is required.",
        ))?;

    let article_html = Html::parse_fragment(article.get("html").unwrap().as_str().unwrap());
    let selector_p = Selector::parse("p").unwrap();
    let meta_description = format!(
        "<meta property=\"og:description\" content=\"{}\">\n",
        handlebars::html_escape(
            article_html
                .select(&selector_p)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .concat()
                .trim()
        ),
    );
    out.write(&meta_description)?;

    let selector_img = Selector::parse("img").unwrap();
    let site_root = ctx.data().get("site_root").unwrap().as_str().unwrap();
    match article_html.select(&selector_img).next() {
        Some(t) => {
            let image_path = handlebars::html_escape(t.value().attr("src").unwrap());
            let image_url = match build_full_url(site_root, &image_path) {
                Ok(u) => u,
                Err(e) => return Err(RenderError::new(format!("{:?}", e))),
            };
            let meta_image = format!("<meta property=\"og:image\" content=\"{}\">\n", image_url,);
            out.write(&meta_image)?;
        }
        None => { /* do nothing */ }
    }

    out.write("<meta property=\"og:type\" content=\"article\">\n")?;
    out.write(&format!(
        "<meta property=\"og:title\" content=\"{}\">\n",
        article.get("title").unwrap().as_str().unwrap(),
    ))?;
    out.write(&format!(
        "<meta property=\"og:url\" content=\"https://mozami.me{}\">\n",
        &article.get("path").unwrap().as_str().unwrap(),
    ))?;

    Ok(())
}

pub fn embed_code(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let path = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderError::new(
            "embed_code: Param 0 with string type is required.",
        ))?;

    match ctx.data().get("codes").unwrap().get(path) {
        Some(code) => {
            out.write(code.get("highlighted_html").unwrap().as_str().unwrap())?;
        }
        None => {
            return Err(RenderError::new(format!(
                "embed_code: There is no code source {}",
                path
            )))
        }
    }

    Ok(())
}

pub fn summarize_article(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let article = h
        .param(0)
        .and_then(|v| v.value().as_object())
        .ok_or(RenderError::new(
            "summarize_article: Param 0 with JSON object type is required.",
        ))?;
    let is_escaped = h
        .param(1)
        .and_then(|v| v.value().as_bool())
        .unwrap_or(false);

    let article_html = Html::parse_fragment(article.get("html").unwrap().as_str().unwrap());
    let selector = Selector::parse("html > *").unwrap();

    for p in article_html.select(&selector).take(4) {
        if is_escaped {
            out.write(&handlebars::html_escape(&p.html()))?;
        } else {
            out.write(&p.html())?;
        }
    }

    Ok(())
}

pub fn time_now(
    _: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    let now = chrono::Utc::now().to_rfc3339();
    out.write(&now)?;
    Ok(())
}

fn build_full_url(site_root: &str, path: &str) -> Result<String, failure::Error> {
    let full_url = url::Url::parse(site_root)?.join(path)?;
    Ok(full_url.as_str().to_string())
}

#[test]
fn test_build_full_url() {
    assert_eq!(
        build_full_url("https://example.com", "/foo/bar.jpg").unwrap(),
        "https://example.com/foo/bar.jpg"
    );
    assert_eq!(
        build_full_url("https://example.com/", "/foo/bar.jpg").unwrap(),
        "https://example.com/foo/bar.jpg"
    );
    assert_eq!(
        build_full_url("https://example.com/", "foo/bar.jpg").unwrap(),
        "https://example.com/foo/bar.jpg"
    );
    assert_eq!(
        build_full_url("https://example.com", "foo/bar.jpg").unwrap(),
        "https://example.com/foo/bar.jpg"
    );
}
