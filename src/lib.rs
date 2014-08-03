#![crate_name = "git2"]
#![crate_type="lib"]



pub mod git2 {

    pub use self::error::_GitError;
    pub use self::repository::{Repository};
    pub use self::reference::{Reference};
    pub use self::oid::{OID, GitOid, ToOID};
    pub use self::object::{Object, GitObjectType};
    pub use self::blob::{Blob, GitOff};
    pub use self::commit::{Commit, GitSignature};
    pub mod error;
    pub mod repository;
    pub mod reference;
    pub mod oid;
    pub mod object;
    pub mod blob;
    pub mod commit;
    pub mod config;

#[link(name="git2")]
    extern {}



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
