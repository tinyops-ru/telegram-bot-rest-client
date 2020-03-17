#[cfg(test)]
mod config_tests {
    use crate::config::config::load_from_file;

    const VALID_CONFIG: &str = "tests/telegram-bot-rest-client.conf";

    #[test]
    fn result_config_should_contain_base_url() {
        match load_from_file(VALID_CONFIG) {
            Ok(config) =>
                assert_eq!(config.base_url, "http://myservar:12345"),
            Err(_error) => panic!("property value expected")
        }
    }

    #[test]
    fn result_config_should_contain_auth_token() {
        match load_from_file(VALID_CONFIG) {
            Ok(config) =>
                assert_eq!(config.auth_token, "SUPPA-MEGA-TOKKEN"),
            Err(_error) => panic!("property value expected")
        }
    }

    #[test]
    fn comments_should_be_ignored() {
        assert_eq!(load_from_file(VALID_CONFIG).is_ok(), true)
    }

    #[test]
    #[should_panic]
    fn return_error_if_base_url_property_is_missing() {
        assert_eq!(load_from_file("tests/missing-base-url.conf").is_err(), true);
    }

    #[test]
    #[should_panic]
    fn return_error_if_auth_token_property_is_missing() {
        assert_eq!(load_from_file("tests/missing-auth-token.conf").is_err(), true);
    }
}
