use std::ptr;
use git2;
use git2::error::{GitError, get_last_error};

pub struct GitOid {
    id: [u8, ..20]
}

pub struct OID {
    _oid: GitOid
}

pub trait ToOID {
    fn to_oid(&self) -> Result<OID, GitError>;
}

impl OID {
    pub fn _new(o: *const GitOid) -> OID { 
        let mut new_oid : GitOid = GitOid{ id: [0,..20]};
        unsafe { ptr::copy_memory(&mut new_oid, o, 1); }
        OID{_oid: new_oid}
    }
    pub fn _get_ptr(&self) -> *const GitOid { &self._oid as *const GitOid }
}

impl PartialEq for OID {
    fn eq(&self, other: &OID) -> bool {
        unsafe { git2::git_oid_cmp(self._get_ptr(), other._get_ptr()) == 0 }
    }
}

impl<'a> ToOID for &'a str {
    fn to_oid(&self) -> Result<OID, GitError> {
        let mut p : GitOid = GitOid{id: [0,..20]};
        let ret = unsafe {
            git2::git_oid_fromstrp(&mut p, self.to_c_str().unwrap())
        };
        if ret != 0 {
            return Err(get_last_error());
        }
        return Ok(OID{_oid: p});
    }
}

impl ToOID for OID {
    fn to_oid(&self) -> Result<OID, GitError> { Ok(*self.clone()) }
}
