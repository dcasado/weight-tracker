use handlebars::TemplateError;

pub fn register(handlebars: &mut handlebars::Handlebars) -> Result<(), TemplateError> {
    handlebars.register_template_file("base", "./templates/base.hbs")?;

    handlebars.register_template_file("index", "./templates/index.hbs")?;

    handlebars.register_template_file("chart", "./templates/chart.hbs")?;

    handlebars.register_template_file("table", "./templates/table.hbs")?;

    handlebars.register_template_file("not_found", "./templates/404.hbs")?;

    Ok(())
}
