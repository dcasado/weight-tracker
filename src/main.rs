use axum::{routing::get_service, Router};
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    Sqlite,
};
use std::str::FromStr;
use tokio::signal;
use tower_http::services::ServeDir;
use weight_tracker::{app_state::AppState, configuration, error::ApiError, routes, templates};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let configuration = configuration::get_configuration();

    if !Sqlite::database_exists(&configuration.database.url)
        .await
        .unwrap_or(false)
    {
        Sqlite::create_database(&configuration.database.url)
            .await
            .expect("Failed to create database");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            SqliteConnectOptions::from_str(&configuration.database.url)
                .expect("Failed to create sqlite options")
                .journal_mode(SqliteJournalMode::Wal),
        )
        .await
        .expect("Failed to create database connection pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to execute migrations");

    let mut handlebars = handlebars::Handlebars::new();

    templates::register(&mut handlebars).map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let app_state = AppState { pool, handlebars };

    let router = Router::new()
        .merge(routes::index(app_state.clone()))
        .merge(routes::chart(app_state.clone()))
        .merge(routes::table(app_state.clone()))
        .nest("/api", routes::api(app_state.clone()))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .fallback_service(routes::not_found(app_state.clone()));

    println!(
        "Listening on {}:{}",
        configuration.application.listen_address, configuration.application.listen_port
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        configuration.application.listen_address, configuration.application.listen_port
    ))
    .await
    .unwrap();

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
