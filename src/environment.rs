pub async fn get_db_url() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::DB_URL).expect(crate::keys::error::DB_URL)
}

pub async fn get_secret_main_key() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::SECRET_MAIN_KEY).expect(crate::keys::error::SECRET_MAIN_KEY)
}

pub async fn get_service_passphrase() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::SERVICE_PASSPHRASE).expect(crate::keys::error::SERVICE_LOGIN)
}

pub async fn get_secret_key() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::SECRET_KEY).expect(crate::keys::error::SECRET_KEY)
}

pub async fn get_root_directory() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::ROOT_DIRECTORY).expect(crate::keys::error::ROOT_DIRECTORY)
}

pub async fn get_icarus_base_api_url() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::ICARUS_BASE_API_URL).expect(crate::keys::error::ICARUS_BASE_API_URL)
}

pub async fn get_icarus_auth_base_api_url() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::ICARUS_AUTH_BASE_API_URL)
        .expect(crate::keys::error::ICARUS_AUTH_BASE_API_URL)
}

pub async fn get_app_env() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::APP_ENV).expect(crate::keys::error::APP_ENV)
}

pub async fn get_backend_port() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::BACKEND_PORT).expect(crate::keys::error::BACKEND_PORT)
}

pub async fn get_frontend_url() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::FRONTEND_URL).expect(crate::keys::error::FRONTEND_URL)
}

pub async fn get_rust_log() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::RUST_LOG).expect(crate::keys::error::RUST_LOG)
}

pub async fn get_allowed_origins() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::ALLOWED_ORIGINS).expect(crate::keys::error::ALLOWED_ORIGINS)
}

/// Get environment not specified in the code
pub async fn get_env(environment: &str) -> String {
    dotenvy::dotenv().ok();
    let my_error = format!("{environment} {}", crate::keys::error::GENERAL_ERROR);
    std::env::var(environment).expect(&my_error)
}
