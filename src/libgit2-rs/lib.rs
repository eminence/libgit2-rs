#[crate_id = "git2rs#0.1"];
#[crate_type="lib"];


#[link(name="git2")]

extern mod extra;

pub mod git2 {
    use std::libc;

    //static lock: Mutex = Mutex::new();

    pub use self::repository::{Repository, GitRepo};
    pub use self::reference::{Reference, GitReference};
    pub use self::oid::{OID, GitOid};
    pub use self::object::{Object, GitObject, GitObjectType};
    pub use self::blob::{Blob, GitBlob};
    pub mod repository;
    pub mod reference;
    pub mod oid;
    pub mod object;
    pub mod blob;

    //use repo::GitRepo;
    //use refe::GitReference;

    #[deriving(Eq,FromPrimitive)]
    enum GitErrorCode {
        GIT_OK = 0,
        GIT_ERROR = -1,
        GIT_ENOTFOUND = -3,
        GIT_EEXISTS = -4,
        GIT_EAMBIGUOUS = -5,
        GIT_EBUFS = -6,
        GIT_EUSER = -7,
        GIT_EBAREREPO = -8,
        GIT_EORPHANEDHEAD = -9,
        GIT_EUNMERGED = -10,
        GIT_ENONFASTFORWARD = -11,
        GIT_EINVALIDSPEC = -12,
        GIT_EMERGECONFLICT = -13,

        GIT_PASSTHROUGH = -30,
        GIT_ITEROVER = -31,
    }

    enum GitErrorType {
        GITERR_NOMEMORY,
        GITERR_OS,
        GITERR_INVALID,
        GITERR_REFERENCE,
        GITERR_ZLIB,
        GITERR_REPOSITORY,
        GITERR_CONFIG,
        GITERR_REGEX,
        GITERR_ODB,
        GITERR_INDEX,
        GITERR_OBJECT,
        GITERR_NET,
        GITERR_TAG,
        GITERR_TREE,
        GITERR_INDEXER,
        GITERR_SSL,
        GITERR_SUBMODULE,
        GITERR_THREAD,
        GITERR_STASH,
        GITERR_CHECKOUT,
        GITERR_FETCHHEAD,
        GITERR_MERGE,
    }

    struct _GitError {
        message: *libc::c_char,
        klass: GitErrorType
    }
    struct GitError {
        message: ~str,
        class: GitErrorType
    }

    pub fn get_last_error() -> GitError {
        let e: *_GitError = unsafe {giterr_last()};
        unsafe {
            match e.to_option() {
                None => fail!("Asked for error, but there was no error"),
                Some(er) => { 
                    GitError{message: ::std::str::raw::from_c_str(er.message), class: er.klass}
                }
            }
        }
    }

    

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
        fn giterr_last() -> *_GitError;

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
        fn git_reference_target(refp: *GitReference) -> *GitOid;
        fn git_reference_name_to_id(oid: *GitOid, repo: *GitRepo, name: *libc::c_char) -> libc::c_int;

        fn git_object_free(obj: *GitObject);
        fn git_object_lookup(obj: **GitObject, repo: *GitRepo, oid: *GitOid, t:GitObjectType) -> libc::c_int;
        fn git_object_type(obj: *GitObject) -> GitObjectType;

        fn git_oid_fromstrp(oid: *GitOid, s: *libc::c_char) -> libc::c_int;

        fn git_blob_free(obj: *GitBlob);
        fn git_blob_lookup(obj: **GitBlob, repo: *GitRepo, oid: *GitOid) -> libc::c_int;
        fn git_blob_rawsize(obj: *GitBlob) -> i64;
        fn git_blob_rawcontent(obj: *GitBlob) -> *u8;
    }
    
    

    


}
