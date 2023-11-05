use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn from(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("usage: minigrep <query> <file path>");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_args_to_config() {
        let args = vec!["".to_string(), "query".to_string(), "file_path".to_string()];
        let config = Config::from(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file_path");
        assert_eq!(config.ignore_case, false);
    }

    #[test]
    fn sets_ignore_case_when_ignore_case_env_var_is_defined() {
        env::set_var("IGNORE_CASE", "any");
        let args = vec!["".to_string(), "query".to_string(), "file_path".to_string()];
        let config = Config::from(&args).unwrap();

        assert!(config.ignore_case);
    }

    #[test]
    fn prints_usage_when_args_are_missing() {
        let args = vec!["".to_string(), "".to_string()];
        let error = Config::from(&args);

        assert!(error.is_err_and(|e| e.contains("usage: minigrep <query> <file path>")));
    }
}