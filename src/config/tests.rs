use super::*;

#[test]
fn default_config_should_have_not_repos() {
    let config = Config::default();
    assert_eq!(config.repos.len(), 0);
}

#[test]
fn test_load_from_path() {
    let path_buf = PathBuf::from("./tests/test_config.toml");
    let config: Config = Config::load_from_path(path_buf.clone());
    assert_eq!(config.repos.len(), 1);
    assert_eq!(config.repos[0].path(), "/path/to/repo");
    assert_eq!(config.repos[0].messages().len(), 1);
    assert_eq!(config.repos[0].messages()[0].message(), "test message");
    let expected_date_string = "2022-01-05T14:00:52-08:00";
    assert_eq!(
        config.repos[0].messages()[0].date().to_string(),
        expected_date_string
    );
}