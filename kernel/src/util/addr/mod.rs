#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VirtAddr(u64);

impl VirtAddr {
    pub fn new(addr: u64) -> VirtAddr {
        VirtAddr(addr)
    }

    pub fn from(addr: impl Into<u64>) -> VirtAddr {
        VirtAddr(addr.into())
    }
}
