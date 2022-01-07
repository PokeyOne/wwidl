use super::*;

/// Simple macro to create an example config with two messages.
macro_rules! create_example_config {
    () => {
        {
            let str_data = "[[repos]]
path = \"/path/to/repo\"

[[repos.messages]]
message = \"test message\"
date = 2022-01-05T14:00:52.000000-08:00

[[repos.messages]]
message = \"test message 2\"
date = 2022-01-05T14:00:53.000000-08:00\n";
            println!("{}", str_data);
            let config: Config = toml::from_str(str_data).unwrap();

            config
        }
    }
}

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

#[test]
fn test_get_last_message() {
    let config = create_example_config!();
    let last_message = config.repo_data(&"/path/to/repo")
        .unwrap()
        .last_message()
        .unwrap();
    assert_eq!(last_message.message(), "test message 2");
    let expected_date_string = "2022-01-05T14:00:53-08:00";
    assert_eq!(
        last_message.date().to_string(),
        expected_date_string
    );
}

#[test]
fn test_current_datetime() {
    // creation shouldn't blow up
    let dt = current_datetime();

    // should not be the same as the first
    let dt2 = current_datetime();
    assert_ne!(dt, dt2);
}

#[test]
fn test_put_note() {
    let mut config = create_example_config!();
    let repo_count_before = config.repos.len();
    config.put_note(&"/path/to/foo/bar/another/repo", "another test message".to_string());
    let repo_count_after = config.repos.len();
    assert_eq!(repo_count_before + 1, repo_count_after);

    let repo_data = config.repo_data(&"/path/to/foo/bar/another/repo").unwrap();
    assert_eq!(repo_data.messages().len(), 1);
    assert_eq!(repo_data.messages()[0].message(), "another test message");
}

#[test]
fn test_remove_note() {
    let mut config = create_example_config!();
    let note_count_before = config.repos[0].messages().len();
    let result = config.remove_notes(&"/path/to/repo", 1);
    let note_count_after = config.repos[0].messages().len();
    assert_eq!(note_count_before - 1, note_count_after);
    assert_eq!(result.len(), 1);

    let repo_data = config.repo_data(&"/path/to/repo");
    assert!(repo_data.is_some());
}

#[test]
fn test_remove_two_notes() {
    let mut config = create_example_config!();
    let result = config.remove_notes(&"/path/to/repo", 2);
    assert_eq!(result.len(), 2);

    let repo_data = config.repo_data(&"/path/to/repo");
    assert!(repo_data.is_none());
}