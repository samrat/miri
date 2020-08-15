// compile-flags: -Zmiri-disable-isolation

#![feature(rustc_private)]

extern crate libc;

fn main() -> std::io::Result<()> {
    let bytes = b"hello\n";
    unsafe {
        libc::write(1, bytes.as_ptr() as *const libc::c_void, 6);
        libc::close(1);
        libc::write(1, bytes.as_ptr() as *const libc::c_void, 6);
    }

    Ok(())
}
