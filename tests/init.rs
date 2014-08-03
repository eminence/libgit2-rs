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

#[test]
fn test_libgit2_caps() {
    let caps = git2::capabilities();
    println!("caps: {}", caps.bits());
    println!("Has Threads: {}", caps.contains(git2::GIT_CAP_THREADS));
    println!("Has SSH: {}", caps.contains(git2::GIT_CAP_SSH));
    println!("Has HTTPS: {}", caps.contains(git2::GIT_CAP_HTTPS));
        
}

#[test]
fn test_libgit2_version() {
    let version = git2::version();
    println!("Version: {}", version);

    assert!(git2::version_check(false) == true);
}
