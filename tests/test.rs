#[cfg(test)]
mod tests {
    #[test]
    fn test_dburl() {
        let result = async_std::task::block_on(icarus_envy::environment::get_db_url());
        assert_eq!(
            result, "postgres://myuser:password@localhost:5432/my_db",
            "DATABASE_URL does not match {:?}",
            result
        );
    }

    #[test]
    fn test_get_secret_main_key() {
        let result = async_std::task::block_on(icarus_envy::environment::get_secret_main_key());
        assert_eq!(
            result, "Somesupersecretpassword!!!45345435",
            "SECRET_MAIN_KEY does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_service_passphrase() {
        let result = async_std::task::block_on(icarus_envy::environment::get_service_passphrase());
        assert_eq!(
            result, "T5OCHDHadAtuOWIqRAS7u8XHDDkzKT1Uvvw7mGMkNzKjVdlHA8xGdILf2adDHspO",
            "SERVICE_PASSPHRASE does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_secret_key() {
        let result = async_std::task::block_on(icarus_envy::environment::get_secret_key());
        assert_eq!(
            result, "AmIGoodEnoughForYou?",
            "SECRET_KEY does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_root_directory() {
        let result = async_std::task::block_on(icarus_envy::environment::get_root_directory());
        assert_eq!(
            result, "/path/to/root",
            "ROOT_DIRECTORY does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_icarus_base_api_url() {
        let result = async_std::task::block_on(icarus_envy::environment::get_icarus_base_api_url());
        assert_eq!(
            result, "https://icarus.com",
            "ICARUS_BASE_API_URL does not match {:?}",
            result
        )
    }
}
