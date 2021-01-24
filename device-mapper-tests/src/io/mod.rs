use crate::{Sector, Stack};
use libc::{c_int, size_t, off64_t};
use core::ffi::c_void;
use cmd_lib::run_fun;
use std::ffi::CString;

pub mod pattern;

pub struct OpenState {
    fd: c_int,
}
impl OpenState {
    fn open(path: &str) -> Self {
        let flags = libc::O_RDWR;
        let path = CString::new(path).unwrap();
        let fd = unsafe { libc::open64(path.as_ptr(), flags) };
        if fd < 0 {
            dbg!(fd);
            dbg!(std::io::Error::last_os_error());
        }
        assert!(fd >= 0);
        Self {
            fd,
        }
    }
    pub fn write(&self, buf: &[u8], offset: Sector, cnt: Sector) {
        let p = unsafe { std::mem::transmute::<*const u8, *const c_void>(buf.as_ptr()) };
        let cnt = cnt.bytes() as size_t;
        let offset = offset.bytes() as off64_t;
        let written = unsafe { libc::pwrite64(self.fd, p, cnt, offset) };
        assert_eq!(written, cnt as isize);
    }
    pub fn read(&self, buf: &mut [u8], offset: Sector, cnt: Sector) {
        let p = unsafe { std::mem::transmute::<*mut u8, *mut c_void>(buf.as_mut_ptr()) };
        let cnt = cnt.bytes() as size_t;
        let offset = offset.bytes() as off64_t;
        let read = unsafe { libc::pread64(self.fd, p, cnt, offset) };
        assert_eq!(read, cnt as isize);
    }
}
impl Drop for OpenState {
    fn drop(&mut self) {
        let r = unsafe { libc::close(self.fd) };
        assert_eq!(r, 0);
    }
}
pub fn open(s: &impl Stack) -> OpenState {
    OpenState::open(&s.path())
}

pub fn test_blk_rw(s: &impl Stack, offset: Sector, cnt: Sector) {
    let rw = open(s);
    let sz = cnt.bytes() as usize;
    let wbuf = vec![1; sz];
    rw.write(&wbuf, offset, cnt);
    let mut rbuf = vec![0; sz];
    rw.read(&mut rbuf, offset, cnt);
    assert_eq!(rbuf, wbuf);
}