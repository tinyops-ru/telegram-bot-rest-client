pub mod config {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    use regex::Regex;

    const BASE_URL_PROPERTY: &str = "base-url";
    const AUTH_TOKEN_PROPERTY: &str = "auth-token";

    #[derive(Clone)]
    pub struct Config {
        pub base_url: String,
        pub auth_token: String
    }

    pub fn load_from_file(file_name: &str) -> Result<Config, io::Error> {
        let input = File::open(file_name).expect("unable to load config from file");
        let buffered = BufReader::new(input);

        let mut base_url: String = String::new();
        let mut auth_token: String = String::new();

        let base_url_regex = get_property_regex(BASE_URL_PROPERTY);
        let auth_token_regex = get_property_regex(AUTH_TOKEN_PROPERTY);

        for line in buffered.lines() {
            let row = line.unwrap();

            if base_url_regex.is_match(&row) {
                let groups_matches = base_url_regex.captures_iter(&row).next();
                let groups = groups_matches.unwrap();
                base_url = String::from(&groups[1]);
            }

            if auth_token_regex.is_match(&row) {
                let groups_matches = auth_token_regex.captures_iter(&row).next();
                let groups = groups_matches.unwrap();
                auth_token = String::from(&groups[1]);
            }
        }

        let config = Config {
            base_url,
            auth_token
        };

        if is_config_valid(&config) {
            Ok(config)

        } else { panic!("invalid configuration") }
    }

    fn get_property_regex(property_name: &str) -> Regex {
        let pattern = format!("^{}=(.*)", property_name);
        return Regex::new(&pattern).unwrap();
    }

    fn is_config_valid(config: &Config) -> bool {
        let mut result = false;

        if config.base_url != "" && config.auth_token != "" {
            result = true

        } else { println!("[!] error: one or more configuration properties are missing") }

        return result;
    }
}
