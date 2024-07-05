pub struct Configuration {
    pub application: ApplicationConfiguration,
    pub database: DatabaseConfiguration,
}

pub struct ApplicationConfiguration {
    pub listen_address: String,
    pub listen_port: String,
}

pub struct DatabaseConfiguration {
    pub url: String,
}

pub fn get_configuration() -> Configuration {
    let listen_address: String = std::env::var("LISTEN_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let listen_port = std::env::var("LISTEN_PORT").unwrap_or("3000".to_string());

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Configuration {
        application: ApplicationConfiguration {
            listen_address,
            listen_port,
        },
        database: DatabaseConfiguration { url: database_url },
    }
}
