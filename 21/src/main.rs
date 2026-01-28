use std::mem::transmute;
unsafe extern "C" { fn write(fd:i32, buf:*const u8, len:usize)->isize; }
fn main(){unsafe{
    let x:u128=transmute([72u8,101,108,108,111,32,119,111,114,108,100,33,10,0,0,0]);
    let b:[u8;16]=transmute(x);
    write(1,b.as_ptr(),13);
}}


