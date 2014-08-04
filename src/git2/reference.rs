extern crate libc;

use std::rc::Rc;
use std::ptr;
use self::libc::{c_int, c_char};

use git2::repository::{Repository};
use git2::repository;
use git2::oid::{OID,GitOid,ToOID};
use git2::error::{GitError,get_last_error};

pub mod opaque {
    pub enum Reference {}
}

extern {
    fn git_reference_free(repf: *mut self::opaque::Reference);
    fn git_reference_lookup(refp: *mut *mut self::opaque::Reference, repo: *mut repository::opaque::Repo, path: *const c_char) -> c_int;
    fn git_reference_is_branch(refp: *mut self::opaque::Reference) -> c_int;
    fn git_reference_is_remote(refp: *mut self::opaque::Reference) -> c_int;
    fn git_reference_type(refp: *mut self::opaque::Reference) -> c_int;
    fn git_reference_target(refp: *mut self::opaque::Reference) -> *const GitOid;
    fn git_reference_name(refp: *const opaque::Reference) -> *const c_char;
    //fn git_reference_name_to_id(oid: *mut GitOid, repo: *mut repository::opaque::Repo, name: *const c_char) -> c_int;
}

struct GitRefPtr {
    _val: *mut self::opaque::Reference
}

#[deriving(Clone)]
pub struct Reference {
    //repo: Repository,
    _ptr: Rc<GitRefPtr>
}


#[deriving(Eq,PartialEq)]
#[allow(non_camel_case_types)]
pub enum GitRefType {
    GIT_REF_INVALID = 0, //* Invalid reference */
    GIT_REF_OID = 1, //* A reference which points at an object id */
    GIT_REF_SYMBOLIC = 2, //* A reference which points at another reference */
    GIT_REF_LISTALL = 1|2 // GIT_REF_OID|GIT_REF_SYMBOLIC,
}

impl Reference {
    pub fn _new(/*repo: &Repository,*/ p: *mut self::opaque::Reference) -> Reference {
        //! Not really public
        Reference{ /*repo : repo.clone(),*/ _ptr: Rc::new(GitRefPtr{_val: p})}
    }

    /// Lookup a reference by name in a repository.
    ///
    /// The name will be checked for validity. See `git_reference_create_symbolic()` for rules about
    /// valid names.
    pub fn lookup(repo: &Repository, name:&str) -> Result<Reference, GitError> {
        unsafe {
            let mut p: *mut self::opaque::Reference = ptr::mut_null();
            let ret = git_reference_lookup(&mut p, repo._get_ptr(), name.to_c_str().unwrap());
            if ret != 0 {
                return Err(get_last_error());
            }
            println!("ref is OK");
            Ok(Reference::_new(p))
        }
    }



    fn _get_ptr(&self) -> *mut self::opaque::Reference {
        self._ptr.deref()._val
    }

    /// Check if a reference is a local branch.
    pub fn is_branch(&self) -> bool {
        unsafe{ git_reference_is_branch(self._get_ptr()) == 1 }
    }

    /// Check if a reference is a remote tracking branch
    pub fn is_remote(&self) -> bool {
        unsafe{ git_reference_is_remote(self._get_ptr()) == 1 }
    }

    /// Get the type of a reference.
    ///
    /// Either direct (`GIT_REF_OID`) or symbolic (`GIT_REF_SYMBOLIC`)
    pub fn get_type(&self) -> GitRefType {
        let enum_val = unsafe {git_reference_type(self._get_ptr())};
        match enum_val {
            1 => GIT_REF_OID,
            2 => GIT_REF_SYMBOLIC,
            _ => fail!("Failed to get ref type")
        }
    }

    /// Get the full name of a reference.
    pub fn name(&self) -> String {
        unsafe {::std::string::raw::from_buf(git_reference_name(self._get_ptr() as *const opaque::Reference) as *const u8)}
    }

    /// Get the OID pointed to by a direct reference.
    ///
    /// Only available if the reference is direct (i.e. an object id reference, not a symbolic
    /// one).
    ///
    /// To find the OID of a symbolic ref, call git_reference_resolve() and then this function (or
    /// maybe use git_reference_name_to_id() to directly resolve a reference name all the way
    /// through to an OID).
    pub fn target(&self) -> Option<OID> {
        let ret : *const GitOid= unsafe {git_reference_target(self._get_ptr())};
        if ret.is_null() { return None; }
        Some(OID::_new(ret))
    }
    
}

impl ToOID for Reference {
    /// Converts this reference to an OID by calling `.target()`
    fn to_oid(&self) -> Result<OID, GitError> {
        Ok(self.target().unwrap())
    }
}

impl Drop for GitRefPtr {
    fn drop(&mut self) {
        println!("dropping this reference!");
        unsafe { git_reference_free(self._val)}
    }
}
