#![crate_name = "git2"]
#![crate_type="lib"]



pub mod git2 {
    extern crate libc;
    use self::libc::{c_char, c_uchar, c_int, c_uint};

    pub use self::error::_GitError;
    pub use self::repository::{Repository, GitRepo};
    pub use self::reference::{Reference, GitReference};
    pub use self::oid::{OID, GitOid, ToOID};
    pub use self::object::{Object, GitObject, GitObjectType};
    pub use self::blob::{Blob, GitBlob, GitOff};
    pub use self::commit::{Commit, GitCommit, GitSignature};
    pub mod error;
    pub mod repository;
    pub mod reference;
    pub mod oid;
    pub mod object;
    pub mod blob;
    pub mod commit;


#[link(name="git2")]
    extern {
        fn giterr_last() -> *mut _GitError;

        fn git_repository_free(repo: *mut GitRepo);
        fn git_repository_init(repo: *mut *mut GitRepo, path: *const c_char, is_bare:u32) -> c_int;
        fn git_repository_open(repo: *mut *mut GitRepo, path: *const c_char) -> c_int;
        fn git_repository_open_bare(repo: *mut *mut GitRepo, path: *const c_char) -> c_int;
        fn git_repository_is_bare(repo: *mut GitRepo) -> c_int;
        fn git_repository_is_empty(repo: *mut GitRepo) -> c_int;
        fn git_repository_is_shallow(repo: *mut GitRepo) -> c_int;
        fn git_repository_path(repo: *mut GitRepo) -> *const c_uchar;

        fn git_reference_free(repf: *mut GitReference);
        fn git_reference_lookup(refp: *mut *mut GitReference, repo: *mut GitRepo, path: *const c_char) -> c_int;
        fn git_reference_is_branch(refp: *mut GitReference) -> c_int;
        fn git_reference_is_remote(refp: *mut GitReference) -> c_int;
        fn git_reference_type(refp: *mut GitReference) -> c_int;
        fn git_reference_target(refp: *mut GitReference) -> *const GitOid;
        fn git_reference_name_to_id(oid: *mut GitOid, repo: *mut GitRepo, name: *const c_char) -> c_int;

        fn git_oid_fromstrp(oid: *mut GitOid, s: *const c_char) -> c_int;
        fn git_oid_cmp(a: *const GitOid, b: *const GitOid) -> c_int;
        fn git_oid_tostr(out: *mut c_char, size: u32, obj: *const GitOid) -> *mut c_char;

        fn git_object_free(obj: *mut GitObject);
        fn git_object_lookup(obj: *mut *mut GitObject, repo: *mut GitRepo, oid: *const GitOid, t:GitObjectType) -> c_int;
        fn git_object_type(obj: *mut GitObject) -> GitObjectType;

        fn git_blob_free(obj: *mut GitBlob);
        fn git_blob_lookup(obj: *mut *mut GitBlob, repo: *mut GitRepo, oid: *const GitOid) -> c_int;
        fn git_blob_rawsize(obj: *const GitBlob) -> GitOff;
        fn git_blob_rawcontent(obj: *mut GitBlob) -> *const u8;
        fn git_blob_owner(obj: *const GitBlob) -> *mut GitRepo;
        fn git_blob_id(obj: *const GitBlob) -> *const GitOid;
        fn git_blob_is_binary(obj: *const GitBlob) -> c_int;

        fn git_commit_free(obj: *mut GitCommit);
        fn git_commit_lookup(obj: *mut *mut GitCommit, repo: *mut GitRepo, oid: *const GitOid) -> c_int;
        fn git_commit_message(obj: *mut GitCommit) -> *const c_char;
        fn git_commit_message_encoding(obj: *mut GitCommit) -> *const c_char;
        fn git_commit_parentcount(obj: *mut GitCommit) -> c_uint;
        fn git_commit_time_offset(obj: *mut GitCommit) -> c_int;
        fn git_commit_time(obj: *mut GitCommit) -> i64;
        fn git_commit_author(obj: *mut GitCommit) -> *const GitSignature;
        fn git_commit_committer(obj: *mut GitCommit) -> *const GitSignature;
    }

}

//pub mod git2 {
//    use std::libc;
//
//    //static lock: Mutex = Mutex::new();
//
//
//    //use repo::GitRepo;
//    //use refe::GitReference;
//
//    #[deriving(Eq,FromPrimitive)]
//    enum GitErrorCode {
//        GIT_OK = 0,
//        GIT_ERROR = -1,
//        GIT_ENOTFOUND = -3,
//        GIT_EEXISTS = -4,
//        GIT_EAMBIGUOUS = -5,
//        GIT_EBUFS = -6,
//        GIT_EUSER = -7,
//        GIT_EBAREREPO = -8,
//        GIT_EORPHANEDHEAD = -9,
//        GIT_EUNMERGED = -10,
//        GIT_ENONFASTFORWARD = -11,
//        GIT_EINVALIDSPEC = -12,
//        GIT_EMERGECONFLICT = -13,
//
//        GIT_PASSTHROUGH = -30,
//        GIT_ITEROVER = -31,
//    }
//
//
//
//    
//
//    pub enum GitOType {
//        
//        GIT_OBJ_ANY = -2,		//< Object can be any of the following */
//        GIT_OBJ_BAD = -1,		//< Object is invalid. */
//        GIT_OBJ__EXT1 = 0,		//< Reserved for future use. */
//        GIT_OBJ_COMMIT = 1,		//< A commit object. */
//        GIT_OBJ_TREE = 2,		//< A tree (directory listing) object. */
//        GIT_OBJ_BLOB = 3,		//< A file revision object. */
//        GIT_OBJ_TAG = 4,		//< An annotated tag object. */
//        GIT_OBJ__EXT2 = 5,		//< Reserved for future use. */
//        GIT_OBJ_OFS_DELTA = 6,  //< A delta, base is given by an offset. */
//        GIT_OBJ_REF_DELTA = 7   //< A delta, base is given by object id. */
//    }
//
//
//
//
//    extern {
//        fn giterr_last() -> *_GitError;
//
//
//
//
//
//
//    }
//    
//    
//
//    
//
//
//}
