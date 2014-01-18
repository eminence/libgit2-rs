
use std::rc::Rc;

use git2;
use git2::repository::{Repository};

pub struct GitReference;
priv struct GitRefPtr {
    priv _val: *GitReference
}

#[deriving(Clone)]
pub struct Reference {
    repo: Repository,
    priv _ptr: Rc<GitRefPtr>
}


#[deriving(Eq)]
pub enum GitRefType {
    GIT_REF_INVALID = 0, //* Invalid reference */
    GIT_REF_OID = 1, //* A reference which points at an object id */
    GIT_REF_SYMBOLIC = 2, //* A reference which points at another reference */
    GIT_REF_LISTALL = 1|2 // GIT_REF_OID|GIT_REF_SYMBOLIC,
}

impl Reference {
    pub fn _new(repo: Repository, p: *GitReference) -> Reference {
        Reference{ repo : repo, _ptr: Rc::new(GitRefPtr{_val: p})}
    }
    fn _get_ptr(&self) -> *GitReference {
        self._ptr.borrow()._val
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
    
}
impl Drop for GitRefPtr {
    fn drop(&mut self) {
        println!("dropping this reference!");
        unsafe { git2::git_reference_free(self._val)}
    }
}