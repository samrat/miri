fn main() {
    let _b = unsafe { std::mem::transmute::<u8, bool>(2) }; //~ ERROR encountered 2, but expected a boolean
}
