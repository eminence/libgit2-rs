extern crate libc;
use self::libc::{c_char, c_int};
use std::ptr;
use std::c_str::CString;

use git2::error::{GitError, get_last_error};
use git2::repository;


extern {
    fn git_clone(repoout: *mut *mut repository::opaque::Repo, url: *const c_char, local_path: *const c_char, options: *const GitCloneOptions) -> c_int;
}

#[allow(dead_code)]
pub struct GitCloneOptions {
    version: uint,

    //checkout_opts: GitCheckoutOpts,
    //remote_callbacks: GitRemoteCallbacks,

    bare: int,
    ignore_cert_errors: int,
    remote_name: CString,
    checkout_branch: CString
}

/// Clone a remote repository.
///
/// If options is None, a default GIT_OPTIONS_INIT will be used
pub fn clone<U: ToCStr, T: ToCStr>(url: U, local_path: T, options: Option<GitCloneOptions>) -> Result<repository::Repository,GitError> {
    if options.is_some() { fail!("Sorry, GitCloneOptions is not suppoted at the moment.  Please pass None for now");}

    let mut repo: *mut repository::opaque::Repo = ptr::mut_null();
    match unsafe { git_clone(&mut repo, url.to_c_str().as_ptr(), local_path.to_c_str().as_ptr(), ptr::null()) } {
        0 => Ok(repository::Repository::_new(repo)),
        _ => Err(get_last_error())
    }


}
