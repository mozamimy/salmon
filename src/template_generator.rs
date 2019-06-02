use failure::Error;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn generate_template(
    matches: &clap::ArgMatches,
    project_dir: &PathBuf,
    article_name: &str,
) -> Result<(), Error> {
    let local_time = chrono::Local::now();
    if is_no_args(matches) || matches.is_present("article") {
        write_article_template(article_name, project_dir, &local_time)?;
    }
    if is_no_args(matches) || matches.is_present("code") {
        create_code_dir(project_dir, &local_time)?;
    }
    if is_no_args(matches) || matches.is_present("image") {
        create_image_dir(project_dir, &local_time)?;
    }

    Ok(())
}

fn is_no_args(matches: &clap::ArgMatches) -> bool {
    !matches.is_present("article") && !matches.is_present("code") && !matches.is_present("image")
}

fn write_article_template(
    article_name: &str,
    project_dir: &PathBuf,
    local_time: &chrono::DateTime<chrono::Local>,
) -> Result<(), Error> {
    let (year, month, day) = decompose_time(local_time);

    let article_dir = project_dir
        .join("articles")
        .join(&year)
        .join(&month)
        .join(&day);
    std::fs::create_dir_all(&article_dir)?;
    log::info!("Created a directory {:?}", article_dir);

    let mut article_path = article_dir.join(article_name);
    article_path.set_extension("md");

    let mut writer = std::io::BufWriter::new(std::fs::File::create(&article_path)?);
    writer.write(b"---\n")?;
    writer.write_fmt(format_args!("title: {}\n", article_name))?;
    writer.write_fmt(format_args!("date: {}-{}-{}\n", year, month, day))?;
    writer.write(b"tags: diary\n")?;
    writer.write(b"---\n\n")?;
    writer.write(b"## Hopping bunnies!\n\n")?;
    writer.write(b"\xF0\x9F\x90\x87 Yay!\n")?;
    log::info!("Wrote an article template to {:?}", article_path);

    Ok(())
}

fn create_code_dir(
    project_dir: &PathBuf,
    local_time: &chrono::DateTime<chrono::Local>,
) -> Result<(), Error> {
    let (year, month, day) = decompose_time(local_time);
    let code_dir = project_dir.join("codes").join(year).join(month).join(day);
    std::fs::create_dir_all(&code_dir)?;
    log::info!("Created a directory {:?}", code_dir);

    Ok(())
}

fn create_image_dir(
    project_dir: &PathBuf,
    local_time: &chrono::DateTime<chrono::Local>,
) -> Result<(), Error> {
    let (year, month, day) = decompose_time(local_time);
    let image_dir = project_dir
        .join("resources")
        .join("images")
        .join(year)
        .join(month)
        .join(day);
    std::fs::create_dir_all(&image_dir)?;
    log::info!("Created a directory {:?}", image_dir);

    Ok(())
}

fn decompose_time(local_time: &chrono::DateTime<chrono::Local>) -> (String, String, String) {
    let year = local_time.format("%Y").to_string();
    let month = local_time.format("%m").to_string();
    let day = local_time.format("%d").to_string();

    (year, month, day)
}
