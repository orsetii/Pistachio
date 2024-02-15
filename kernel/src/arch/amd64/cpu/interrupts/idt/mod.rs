use entry::Entry;
use crate::util::addr::VirtAddr;
use core::{arch::asm, fmt};

lazy_static::lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.entries[DIVIDE_BY_ZERO].set_handler_fn(exceptions::divide_by_zero);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub mod entry;
pub mod exceptions;

#[repr(C, align(16))]
pub struct Idt {
    pub entries: [Entry; 32],
}

impl Idt {
    pub fn new() -> Idt {
        Idt {
            entries: [Entry::missing(); 32],
        }
    }

            pub fn load(&'static self) {
        use core::mem::size_of;

        let ptr = Idtr {
            base: VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { Self::lidt(&ptr) };
    }

    #[inline]
    unsafe fn lidt(idt: &Idtr) {
        unsafe {
            asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
        }
    }
}

const DIVIDE_BY_ZERO: usize = 0;

#[repr(C, packed)]
pub struct Idtr {
    limit: u16,
    base: VirtAddr,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SegmentSelector(u16);


pub type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);

#[repr(C)]
pub struct InterruptStackFrame {
    value: InterruptStackFrameValue,
}

impl core::ops::Deref for InterruptStackFrame {
    type Target = InterruptStackFrameValue;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl fmt::Debug for InterruptStackFrame {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// Represents the interrupt stack frame pushed by the CPU on interrupt or exception entry.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrameValue {
    /// This value points to the instruction that should be executed when the interrupt
    /// handler returns. For most interrupts, this value points to the instruction immediately
    /// following the last executed instruction. However, for some exceptions (e.g., page faults),
    /// this value points to the faulting instruction, so that the instruction is restarted on
    /// return. See the documentation of the [`InterruptDescriptorTable`] fields for more details.
    pub instruction_pointer: VirtAddr,
    /// The code segment selector, padded with zeros.
    pub code_segment: u64,
    /// The flags register before the interrupt handler was invoked.
    pub cpu_flags: u64,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: VirtAddr,
    /// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
    pub stack_segment: u64,
}