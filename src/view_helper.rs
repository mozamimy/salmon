use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

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
            "Param 0 with string type is required for convert_to_iso8601 helper.",
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
