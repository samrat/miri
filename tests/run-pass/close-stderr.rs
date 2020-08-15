// compile-flags: -Zmiri-disable-isolation

#![feature(rustc_private)]

extern crate libc;

fn main() -> std::io::Result<()> {
    let bytes = b"hello\n";
    unsafe {
        libc::write(2, bytes.as_ptr() as *const libc::c_void, 6);
        libc::close(2);
        libc::write(2, bytes.as_ptr() as *const libc::c_void, 6);
    }

    Ok(())
}
