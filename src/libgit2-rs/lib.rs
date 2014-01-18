#[crate_id = "git2rs#0.1"];
#[crate_type="lib"];


#[link(name="git2")]

extern mod extra;

pub mod git2 {
    use std::libc;

    //static lock: Mutex = Mutex::new();

    pub use self::repository::{Repository, GitRepo};
    pub use self::reference::{Reference, GitReference};
    pub mod repository;
    pub mod reference;

    //use repo::GitRepo;
    //use refe::GitReference;


    type rawGitOIDPtr = *libc::c_void;


    
    struct OID;

    pub enum GitOType {
        
        GIT_OBJ_ANY = -2,		//< Object can be any of the following */
        GIT_OBJ_BAD = -1,		//< Object is invalid. */
        GIT_OBJ__EXT1 = 0,		//< Reserved for future use. */
        GIT_OBJ_COMMIT = 1,		//< A commit object. */
        GIT_OBJ_TREE = 2,		//< A tree (directory listing) object. */
        GIT_OBJ_BLOB = 3,		//< A file revision object. */
        GIT_OBJ_TAG = 4,		//< An annotated tag object. */
        GIT_OBJ__EXT2 = 5,		//< Reserved for future use. */
        GIT_OBJ_OFS_DELTA = 6,  //< A delta, base is given by an offset. */
        GIT_OBJ_REF_DELTA = 7   //< A delta, base is given by object id. */
    }


    extern {
        #[link(name="achin")];
        fn git_repository_free(repo: *GitRepo);
        fn git_repository_init(repo: **GitRepo, path: *libc::c_char, is_bare:u32) -> libc::c_int;
        fn git_repository_open(repo: **GitRepo, path: *libc::c_char) -> libc::c_int;
        fn git_repository_open_bare(repo: **GitRepo, path: *libc::c_char) -> libc::c_int;
        fn git_repository_is_bare(repo: *GitRepo) -> libc::c_int;
        fn git_repository_is_empty(repo: *GitRepo) -> libc::c_int;
        fn git_repository_is_shallow(repo: *GitRepo) -> libc::c_int;
        fn git_repository_path(repo: *GitRepo) -> *libc::c_char;

        fn git_reference_free(repf: *GitReference);
        fn git_reference_lookup(refp: **GitReference, repo: *GitRepo, path: *libc::c_char) -> libc::c_int;
        fn git_reference_is_branch(refp: *GitReference) -> libc::c_int;
        fn git_reference_is_remote(refp: *GitReference) -> libc::c_int;
        fn git_reference_type(refp: *GitReference) -> libc::c_int;
        fn git_reference_target(refp: *GitReference) -> rawGitOIDPtr;
    }
    

    


}
