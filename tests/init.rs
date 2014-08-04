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
fn test_clone_https() {

    let dir =  TempDir::new("git2_test1").unwrap();
    println!("Cloning into {}", dir.path().display());

    let result = git2::clone("https://github.com/eminence/libgit2-rs.git", dir.path(), None);
    let repo = match result {
        Err(e) => fail!("Failed to clone! {}", e),
        Ok(r) => r
    };

    assert!(repo.is_bare() == false);
}
#[test]
fn test_clone_http() {

    let dir =  TempDir::new("git2_test1").unwrap();
    println!("Cloning into {}", dir.path().display());

    let result = git2::clone("http://github.com/eminence/libgit2-rs.git", dir.path(), None);
    let repo = match result {
        Err(e) => fail!("Failed to clone! {}", e),
        Ok(r) => r
    };

    assert!(repo.is_bare() == false);
}
#[test]
fn test_clone_git() {
    use git2::git2::ToOID;

    let dir =  TempDir::new("git2_test1").unwrap();
    println!("Cloning into {}", dir.path().display());

    let result = git2::clone("git://github.com/eminence/libgit2-rs.git", dir.path(), None);
    let repo = match result {
        Err(e) => fail!("Failed to clone! {}", e),
        Ok(r) => r
    };

    assert!(repo.is_bare() == false);
    let commit = repo.lookup_commit("444ff52ddb1f2f4b6a5627d57a5dd6d5a9fc86fd").unwrap();
    println!("commit.message() -->{}<--", commit.message());
    assert!(commit.message().as_slice() == "Added capability and version check\n");
    assert!(commit.author().name.as_slice() == "Andrew Chin");
    assert!(commit.author().email.as_slice() == "achin@eminence32.net");
    assert!(commit.parentcount() == 1);
    let parent = match commit.parent(0) {
        Err(r) => fail!("Failed to get parent! {}", r),
        Ok(p) => p
    };
    assert!(parent.id().to_string().as_slice() == "e7abafb574c707ba9906e90c1dc03126bab98973");
    assert!(parent.id() == "e7abafb574c707ba9906e90c1dc03126bab98973".to_oid().unwrap());



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
