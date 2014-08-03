extern crate libc;

use std::ptr;
use std::fmt::{Show, Formatter, FormatError};
use self::libc::{c_char, c_int};

use git2::error::{GitError, get_last_error};

extern {
    fn git_oid_fromstrp(oid: *mut GitOid, s: *const c_char) -> c_int;
    fn git_oid_cmp(a: *const GitOid, b: *const GitOid) -> c_int;
    fn git_oid_tostr(out: *mut c_char, size: u32, obj: *const GitOid) -> *mut c_char;
}

/// Size (in bytes) of a raw/binary oid
pub static GIT_OID_RAWSZ: uint = 20;
/// Size (in bytes) of a hex formattted oid
pub static GIT_OID_HEXSZ: uint =  (GIT_OID_RAWSZ * 2);

#[allow(dead_code)]
pub struct GitOid {
    id: [u8, ..20]
}

pub struct OID {
    _oid: GitOid
}

/// A trait for anything that can be converted into an OID
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

    /// Format a OID as a hex-formatted String
    pub fn to_string(&self) -> String {
        let mut s = ::std::string::String::new();
        s.grow(GIT_OID_HEXSZ + 1, '+');
        assert!(s.len() == GIT_OID_HEXSZ + 1);
        let mut cstr = s.to_c_str();
        unsafe {
            git_oid_tostr(cstr.as_mut_ptr(), GIT_OID_HEXSZ as u32 + 1u32, self._get_ptr());
        }
        match cstr.as_str() {
            None => fail!("Failed to get str!"),
            Some(st) => st.into_string()
        }
    }

}

impl Show for OID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(self.to_string().as_bytes())
    }
}

impl PartialEq for OID {
    fn eq(&self, other: &OID) -> bool {
        unsafe { git_oid_cmp(self._get_ptr(), other._get_ptr()) == 0 }
    }
}

impl<'a> ToOID for &'a str {
    fn to_oid(&self) -> Result<OID, GitError> {
        let mut p : GitOid = GitOid{id: [0,..20]};
        let ret = unsafe {
            git_oid_fromstrp(&mut p, self.to_c_str().unwrap())
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
