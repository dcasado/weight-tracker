use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub handlebars: handlebars::Handlebars<'static>,
}
