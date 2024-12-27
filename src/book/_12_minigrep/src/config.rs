use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = args.next().ok_or("usage: minigrep <query> <file path>")?;
        let file_path = args.next().ok_or("usage: minigrep <query> <file path>")?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn parses_args_to_config() {
        env::remove_var("IGNORE_CASE");
        let args = ["", "query", "file_path"]
            .into_iter()
            .map(|s| s.to_string());
        let config = Config::from(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file_path");
        assert_eq!(config.ignore_case, false);
    }

    #[test]
    #[serial]
    fn sets_ignore_case_when_ignore_case_env_var_is_defined() {
        env::set_var("IGNORE_CASE", "any");
        let args = ["", "query", "file_path"]
            .into_iter()
            .map(|s| s.to_string());
        let config = Config::from(args).unwrap();

        assert!(config.ignore_case);
    }

    #[test]
    fn prints_usage_when_args_are_missing() {
        let args = [""].into_iter().map(|s| s.to_string());
        let error = Config::from(args);
        assert!(error.is_err_and(|e| e.contains("usage: minigrep <query> <file path>")));

        let args = ["", ""].into_iter().map(|s| s.to_string());
        let error = Config::from(args);
        assert!(error.is_err_and(|e| e.contains("usage: minigrep <query> <file path>")));
    }
}
