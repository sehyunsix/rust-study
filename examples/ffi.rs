use std::ffi::CString;
use std::os::raw::c_char;
//synkk
unsafe extern "C" {
    //함수를 구현할 필요 x
    fn puts(s: *const c_char) -> i32;
}
fn main() {
    let s = CString::new("Hello").unwrap();
    unsafe {
        puts(s.as_ptr());
    }
}
