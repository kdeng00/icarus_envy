// Environment key for Database management
pub const DB_URL: &str = "DATABASE_URL";

// Environment key for secret main key
// Used for the icarus app
pub const SECRET_MAIN_KEY: &str = "SECRET_MAIN_KEY";

// Environment key for service logins
pub const SERVICE_PASSPHRASE: &str = "SERVICE_PASSPHRASE";

// Environment key for secret key
// Generic use of secret key that could be found in various apps
pub const SECRET_KEY: &str = "SECRET_KEY";

// Environment key for root directory for the icarus app
pub const ROOT_DIRECTORY: &str = "ROOT_DIRECTORY";

// Environment key for icarus api url
pub const ICARUS_BASE_API_URL: &str = "ICARUS_BASE_API_URL";

// Environment key for icarus auth api url
pub const ICARUS_AUTH_BASE_API_URL: &str = "ICARUS_AUTH_BASE_API_URL";

pub mod error {
    use const_format::concatcp;

    pub const GENERAL_ERROR: &str = "must not be set in enviornment file";
    pub const DB_URL: &str = concatcp!(super::DB_URL, " ", GENERAL_ERROR);
    pub const SECRET_MAIN_KEY: &str = concatcp!(super::SECRET_MAIN_KEY, " ", GENERAL_ERROR);
    pub const SERVICE_LOGIN: &str = concatcp!(super::SERVICE_PASSPHRASE, " ", GENERAL_ERROR);
    pub const SECRET_KEY: &str = concatcp!(super::SECRET_KEY, " ", GENERAL_ERROR);
    pub const ROOT_DIRECTORY: &str = concatcp!(super::ROOT_DIRECTORY, " ", GENERAL_ERROR);
    pub const ICARUS_BASE_API_URL: &str = concatcp!(super::ICARUS_BASE_API_URL, " ", GENERAL_ERROR);
    pub const ICARUS_AUTH_BASE_API_URL: &str =
        concatcp!(super::ICARUS_AUTH_BASE_API_URL, " ", GENERAL_ERROR);
}
