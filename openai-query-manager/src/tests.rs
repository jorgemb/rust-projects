use std::env;
use util::test_helper;
use super::*;

#[test]
fn test_manager_initialization() {
    // Create temporary directory
    let temp_directory = test_helper::TempDirectoryHandler::build().unwrap();
    env::set_current_dir(temp_directory.path()).expect("Couldn't change directory");

    // Create manager
    let query_manager = QueryManager::build().expect("Couldn't create manager");

    // Check that all directories exist
    let mut path = env::current_dir().expect("Couldn't get CWD");
    path.push(QueryManager::QUERY_DIRECTORY);

    assert!(path.exists(), "Path created for queries: {:?}", path);
    assert_eq!(*query_manager.base_path(), path);

    for directory in QueryManager::STORAGE_DIRECTORIES {
        path.push(directory);
        assert!(path.exists(), "Path for storage: {:?}", path);
        assert_eq!(*query_manager.storage_paths(directory).expect("Expected storage"), path);
        path.pop();
    }
}