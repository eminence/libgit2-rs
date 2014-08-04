#![crate_name = "git2"]
#![crate_type="lib"]


//! The git2 crate

pub mod git2 {
    //! The git2 module

    extern crate libc;
    use self::libc::c_int;

    pub use self::repository::{Repository};
    pub use self::reference::{Reference};
    pub use self::oid::{OID, ToOID};
    pub use self::object::{Object, GitObjectType};
    pub use self::blob::{Blob, GitOff};
    pub use self::commit::{Commit};
    pub use self::clone::clone;
    pub mod error;
    pub mod repository;
    pub mod reference;
    pub mod oid;
    pub mod object;
    pub mod blob;
    pub mod commit;
    pub mod config;
    pub mod clone;
    pub mod branch;


    bitflags!(flags CapabilityFlags: u32 {
            static GIT_CAP_THREADS = (1 << 0),
            static GIT_CAP_HTTPS = (1 << 1),
            static GIT_CAP_SSH = (1 << 2)
        })

    #[deriving(Show)]
    pub struct Version {
        major: i32,
        minor: i32,
        rev: i32
    }


#[link(name="git2")]
    extern {
        fn git_libgit2_capabilities() -> c_int;
        fn git_libgit2_version(major: *mut c_int, minor: *mut c_int, rev: *mut c_int);
    }

    pub fn capabilities() -> CapabilityFlags {
        let caps = unsafe {git_libgit2_capabilities() as u32};
        CapabilityFlags::from_bits(caps).unwrap()
    }

    /// Returns the version of your libgit2 library
    pub fn version() -> Version {
        let mut major = 0;
        let mut minor = 0;
        let mut rev = 0;
        unsafe {git_libgit2_version(&mut major, &mut minor, &mut rev)};
        Version{major: major, minor: minor, rev: rev}
    }

    /// Checks to make sure your version of libgit2 is appropriate
    ///
    /// If fail is true, this function will fail instead of returning false
    pub fn version_check(fail: bool) -> bool {
        let version = version();
        
        if ! (version.major == 0 && version.minor == 20) {
            if fail { fail!("Incorrect libgit2 version!"); }
            return false;
        }
        true

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
