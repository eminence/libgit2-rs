#![link(name="git2")]

extern crate git2;
use git2::git2;
use std::io::TempDir;

#[test]
fn test_object() {

    let repo = match git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/src/libgit2-rs/test_data/repoA")) {
        Ok(r) => r,
        Err(e) => fail!("Failed to open repo:\n{}", e.message)
    };
    assert!(repo.is_empty() == false);

    let obj = repo.lookup_object("437c71f50f7de5cc6982adc2d3f680318a42e0ec", git2::object::GIT_OBJ_ANY).unwrap();
    assert!(obj.get_type() == git2::object::GIT_OBJ_TREE);

    let obj2 = repo.lookup_object("8717d8cfa524fd42ae3f8f3efc00bb5cace3af17", git2::object::GIT_OBJ_ANY).unwrap();
    assert!(obj2.get_type() == git2::object::GIT_OBJ_COMMIT);

    let obj3 = repo.lookup_object("95d09f2b10159347eece71399a7e2e907ea3df4f", git2::object::GIT_OBJ_ANY).unwrap();
    assert!(obj3.get_type() == git2::object::GIT_OBJ_BLOB);

}
