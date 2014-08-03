#![link(name="git2")]

extern crate git2;
use git2::git2;

#[test]
fn test_config() {
    let repo = match git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/tests/data/repoA")) {
        Ok(r) => r,
        Err(e) => fail!("Failed to open repo:\n{}", e.message)
    };
    assert!(repo.is_empty() == false);

    let config = match repo.config() {
        Err(e) => fail!("Failed to get config! {}", e),
        Ok(c) => c
    };


    assert!(config.get_bool("core.bare").is_ok());
    assert!(config.get_bool("core.doesnotexist").is_err());
    assert!(config.get_bool("core.bare").unwrap() == repo.is_bare());

    let entryR = config.get_entry("core.bare");
    assert!(entryR.is_ok());
    let entry = entryR.unwrap();
    assert!(entry.name.as_slice() == "core.bare");
    assert!(entry.value.as_slice() == repo.is_bare().to_string().as_slice());

    println!("{}", config.get_entry("user.name"));

    for entry in config.iterator().unwrap() {
        println!("have item: {}", entry);
    }

}
