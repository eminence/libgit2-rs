#![link(name="git2")]

extern crate git2;
use git2::git2;
use std::io::TempDir;

#[test]
fn test_init_01() {

    let dir = TempDir::new("git2_test1").unwrap();
    println!("new test repo in {}", dir.path().display());
    let bare = false;
    let repo = match git2::Repository::init(dir.path(), bare) {
        Ok(r) => r,
        Err(e) => fail!("Failed to init repo:\n{}", e.message)
    };
    assert!(repo.is_bare() == bare);
    assert!(repo.is_empty() == true);
}

#[test]
fn test_init_02() {

    let dir = TempDir::new("git2_test1").unwrap();
    println!("new test repo in {}", dir.path().display());
    let bare = true;
    let repo = match git2::Repository::init(dir.path(), bare) {
        Ok(r) => r,
        Err(e) => fail!("Failed to init repo:\n{}", e.message)
    };
    assert!(repo.is_bare() == bare);
    assert!(repo.is_empty() == true);
    assert!(repo.is_shallow() == false);
    let path = repo.path();
    println!("{}", path.display());
    assert!(&path == dir.path());
}
