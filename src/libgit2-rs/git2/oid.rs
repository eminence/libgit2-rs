use std::ptr;
use std::from_str::FromStr;
use git2;
use git2::GitError;

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
    pub fn _new(o: *GitOid) -> OID { 
        let mut new_oid : GitOid = GitOid{ id: [0,..20]};
        unsafe { ptr::copy_memory(ptr::to_mut_unsafe_ptr(&mut new_oid), o, 1); }
        OID{_oid: new_oid}
    }
    pub fn _get_ptr(&self) -> *GitOid { ptr::to_unsafe_ptr(&self._oid) }
}

impl<'a> ToOID for &'a str {
    fn to_oid(&self) -> Result<OID, GitError> {
        let p : GitOid = GitOid{id: [0,..20]};
        let ret = unsafe {
            git2::git_oid_fromstrp(ptr::to_unsafe_ptr(&p), self.to_c_str().unwrap())
        };
        if ret != 0 {
            return Err(git2::get_last_error());
        }
        return Ok(OID{_oid: p});
    }
}

impl ToOID for OID {
    fn to_oid(&self) -> Result<OID, GitError> { Ok(*self.clone()) }
}
