extern crate libc;

use std::rc::Rc;
use self::libc::{c_int};

use git2::repository::{Repository};
use git2::oid::{OID,GitOid,ToOID};
use git2::error::{GitError};

extern {
    fn git_reference_free(repf: *mut GitReference);
    //fn git_reference_lookup(refp: *mut *mut GitReference, repo: *mut GitRepo, path: *const c_char) -> c_int;
    fn git_reference_is_branch(refp: *mut GitReference) -> c_int;
    fn git_reference_is_remote(refp: *mut GitReference) -> c_int;
    fn git_reference_type(refp: *mut GitReference) -> c_int;
    fn git_reference_target(refp: *mut GitReference) -> *const GitOid;
    //fn git_reference_name_to_id(oid: *mut GitOid, repo: *mut GitRepo, name: *const c_char) -> c_int;
}

pub struct GitReference;
struct GitRefPtr {
    _val: *mut GitReference
}

#[deriving(Clone)]
pub struct Reference {
    repo: Repository,
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
    pub fn _new(repo: Repository, p: *mut GitReference) -> Reference {
        Reference{ repo : repo, _ptr: Rc::new(GitRefPtr{_val: p})}
    }
    fn _get_ptr(&self) -> *mut GitReference {
        self._ptr.deref()._val
    }
    pub fn is_branch(&self) -> bool {
        unsafe{ git_reference_is_branch(self._get_ptr()) == 1 }
    }
    pub fn is_remote(&self) -> bool {
        unsafe{ git_reference_is_remote(self._get_ptr()) == 1 }
    }
    pub fn get_type(&self) -> GitRefType {
        let enum_val = unsafe {git_reference_type(self._get_ptr())};
        match enum_val {
            1 => GIT_REF_OID,
            2 => GIT_REF_SYMBOLIC,
            _ => fail!("Failed to get ref type")
        }
    }
    pub fn target(&self) -> Option<OID> {
        let ret : *const GitOid= unsafe {git_reference_target(self._get_ptr())};
        if ret.is_null() { return None; }
        Some(OID::_new(ret))
    }
    
}

impl ToOID for Reference {
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
