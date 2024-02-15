use crate::println;

use super::InterruptStackFrame;

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}


pub extern "x86-interrupt" fn divide_by_zero(sf: InterruptStackFrame)  {
    println!("{:#?}", sf);
    loop {}
}