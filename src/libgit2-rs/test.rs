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
    let x = git2::Repository::init(dir.path(), bare);
    assert!(x.is_bare() == bare);
    assert!(x.is_empty() == true);
}

#[test]
fn test_init_02() {
    use extra::tempfile::TempDir;

    let dir = TempDir::new("git2_test1").unwrap();
    println!("new test repo in {}", dir.path().display());
    let bare = true;
    let x = git2::Repository::init(dir.path(), bare);
    assert!(x.is_bare() == bare);
    assert!(x.is_empty() == true);
    assert!(x.is_shallow() == false);
    let path = x.path();
    println!("{}", path.display());
    assert!(&path == dir.path());
}

#[test]
fn test_lookup_reference() {
    let repo = git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/src/libgit2/test_data/repoA"));
    assert!(repo.is_empty() == false);
    let refe = repo.lookup_reference("refs/heads/master");
    assert!(refe.is_remote() == false);
    assert!(refe.is_branch() == true);
    assert!(refe.get_type() == git2::reference::GIT_REF_OID);
}
