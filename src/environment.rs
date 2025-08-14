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
