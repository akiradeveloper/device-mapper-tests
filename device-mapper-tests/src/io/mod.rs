use crate::{Sector, Stack};
use cmd_lib::run_fun;
use core::ffi::c_void;
use libc::{c_int, off64_t, size_t};
use std::ffi::CString;

pub mod pattern;

pub struct OpenState {
    fd: c_int,
}
impl OpenState {
    fn open(path: &str) -> Self {
        assert_eq!(crate::blkdev::get_blocksize(path), 512);

        let flags = libc::O_RDWR | libc::O_DIRECT;
        let path = CString::new(path).unwrap();
        let fd = unsafe { libc::open64(path.as_ptr(), flags) };
        if fd < 0 {
            dbg!(fd);
            dbg!(std::io::Error::last_os_error());
        }
        assert!(fd >= 0);
        Self { fd }
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

// Allocate buffer which is 512 bytes aligned.
// In direct IO, the buffer must be aligned in block size.
pub fn buf(src: &[u8]) -> Vec<u8> {
    use std::alloc::{alloc, Layout};

    let sz = src.len();
    let layout = Layout::from_size_align(sz, 512).unwrap();
    let p = unsafe { alloc(layout) };
    let mut buf = unsafe { Vec::from_raw_parts(p, sz, sz) };
    buf.copy_from_slice(&src);
    buf
}

pub fn test_blk_rw(s: &impl Stack, offset: Sector, cnt: Sector) {
    let rw = open(s);
    let sz = cnt.bytes() as usize;
    let wbuf = buf(&vec![1; sz]);
    rw.write(&wbuf, offset, cnt);
    let mut rbuf = buf(&vec![0; sz]);
    rw.read(&mut rbuf, offset, cnt);
    assert_eq!(rbuf, wbuf);
}
