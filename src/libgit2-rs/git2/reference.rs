
use std::rc::Rc;

use git2;
use git2::repository::{Repository};
use git2::oid::{OID,GitOid,ToOID};
use git2::error::{GitError, get_last_error};

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
        unsafe{ git2::git_reference_is_branch(self._get_ptr()) == 1 }
    }
    pub fn is_remote(&self) -> bool {
        unsafe{ git2::git_reference_is_remote(self._get_ptr()) == 1 }
    }
    pub fn get_type(&self) -> GitRefType {
        let enum_val = unsafe {git2::git_reference_type(self._get_ptr())};
        match enum_val {
            1 => GIT_REF_OID,
            2 => GIT_REF_SYMBOLIC,
            _ => fail!("Failed to get ref type")
        }
    }
    pub fn target(&self) -> Option<OID> {
        let ret : *const GitOid= unsafe {git2::git_reference_target(self._get_ptr())};
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
        unsafe { git2::git_reference_free(self._val)}
    }
}
