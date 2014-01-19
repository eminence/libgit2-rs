extern mod git2rs;
extern mod extra;
use git2rs::git2;

#[link(name="git2")]
#[link_args="-lgit2"]

#[test]
fn test_init_01() {
    use extra::tempfile::TempDir;

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
    use extra::tempfile::TempDir;

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
fn test_lookup_reference() {
    let repo = match git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/src/libgit2-rs/test_data/repoA")) {
        Ok(r) => r,
        Err(e) => fail!("Failed to open repo:\n{}", e.message)
    };
    assert!(repo.is_empty() == false);
    let refe = match repo.lookup_reference("refs/heads/master") {
        Ok(r) => r,
        Err(e) => fail!("Failed to get reference:\n{}", e.message)
    };
    assert!(refe.is_remote() == false);
    assert!(refe.is_branch() == true);
    assert!(refe.get_type() == git2::reference::GIT_REF_OID);
    
    if repo.lookup_reference("refs/heads/noexist").is_ok() {
        fail!("Found repo, but wasn't supposed to");
    }
}
