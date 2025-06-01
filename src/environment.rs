
pub async fn get_db_url() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::DBURL).expect(crate::keys::error::ERROR)
}

pub async fn get_secret_main_key() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::SECRET_MAIN_KEY).expect(crate::keys::error::SECRET_MAIN_KEY)
}

pub async fn get_secret_key() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::SECRET_KEY).expect(crate::keys::error::SECRET_KEY)
}

pub async fn get_root_directory() -> String {
    dotenvy::dotenv().ok();
    std::env::var(crate::keys::ROOT_DIRECTORY).expect(crate::keys::error::ROOT_DIRECTORY)
}
