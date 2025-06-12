// Environment key for Database management
pub const DB_URL: &str = "DATABASE_URL";

// Environment key for secret main key
// Used for the icarus app
pub const SECRET_MAIN_KEY: &str = "SECRET_MAIN_KEY"; 

// Environment key for secret key
// Generic use of secret key that could be found in various apps
pub const SECRET_KEY: &str = "SECRET_KEY";

// Environment key for root directory for the icarus app
pub const ROOT_DIRECTORY: &str = "ROOT_DIRECTORY";

// Environment key for icarus api url
pub const ICARUS_BASE_API_URL: &str = "ICARUS_BASE_API_URL";

pub mod error {
    pub const DB_URL: &str = "DATABASE_URL must be set in .env";
    pub const SECRET_KEY: &str = "SECRET_KEY must be set in environment file";
    pub const SECRET_MAIN_KEY: &str = "SECRET_MAIN_KEY must not be set in environment file";
    pub const ROOT_DIRECTORY: &str = "ROOT_DIRECTORY must not be set in environment file";
    pub const ICARUS_BASE_API_URL: &str = "ICARUS_BASE_API_URL must not be set in enviornment file";
}
