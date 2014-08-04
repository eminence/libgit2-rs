extern crate libc;

use self::libc::{c_int, c_char};
use std::ptr;
use std::iter::Iterator;

use git2::{reference, repository};
use git2::error::{GitError,get_last_error};

pub mod opaque {
    pub enum BranchIterator {}
}

extern {
    fn git_branch_lookup(out: *mut *mut reference::opaque::Reference, repo: *mut repository::opaque::Repo, name: *const c_char, btype: GitBranchType) -> c_int;
    fn git_branch_iterator_new(out: *mut *mut opaque::BranchIterator, repo: *mut repository::opaque::Repo, flags: GitBranchType) -> c_int;
    fn git_branch_next(out: *mut *mut reference::opaque::Reference, out_flags: *mut GitBranchType, iter: *mut opaque::BranchIterator) -> c_int;
    fn git_branch_iterator_free(obj: *mut opaque::BranchIterator);
}

#[repr(C)]
pub enum GitBranchType {
    GIT_BRANCH_LOCAL = 1,
    GIT_BRANCH_REMOTE = 2,
    /// A logical OR of GIT_BRANCH_LOCLA and GIT_BRANCH_REMOTE, usful in `branch::iterator`
    GIT_BRANCH_LOCALREMOTE = 3
}


/// An iterator over the branches in the repository (local or remote)
pub struct GitBranchIterator {
    _iter: *mut self::opaque::BranchIterator,
}

impl Iterator<reference::Reference> for GitBranchIterator {
    fn next(&mut self) -> Option<reference::Reference> {
        let mut refe: *mut reference::opaque::Reference = ptr::mut_null();
        let mut flags = GIT_BRANCH_LOCAL;
        let ret = unsafe { git_branch_next(&mut refe, &mut flags, self._iter) };
        if ret == 0 {
            return Some(reference::Reference::_new(refe));
        } else if ret == -31 { // ITEROVER
            return None;
        } else {
            fail!("git_branch_next failed! {}", get_last_error());
        }
    }
}


/// Lookup a branch by its name in a repository.
pub fn lookup(repo: &repository::Repository, name: &str, btype: GitBranchType) -> Result<Option<reference::Reference>, GitError> {
    let mut refe: *mut reference::opaque::Reference = ptr::mut_null();
    let ret = unsafe { git_branch_lookup(&mut refe, repo._get_ptr(), name.to_c_str().as_ptr(), btype) };
    if ret == 0 {
        return Ok(Some(reference::Reference::_new(refe)));
    } else if ret == -3 { // not founc 
        return Ok(None);
    } else {
        return Err(get_last_error());
    }
}

/// Get an iterator over all the branches in this repository
pub fn iterator(repo: &repository::Repository, flags: GitBranchType) -> Result<GitBranchIterator,GitError> {
    let mut iter: *mut opaque::BranchIterator = ptr::mut_null();
    match unsafe { git_branch_iterator_new(&mut iter, repo._get_ptr(), flags) } {
        0 => Ok(GitBranchIterator{_iter: iter}),
        _ => Err(get_last_error())
    }

}
impl Drop for GitBranchIterator {
    fn drop(&mut self) {
        println!("Dropping this branch iterator!");
        unsafe {git_branch_iterator_free(self._iter)} 
    }
}
