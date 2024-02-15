use bootloader_api::{config::Mapping, BootloaderConfig};

use crate::{ print, println};

pub mod cpu;
pub mod drivers;
pub mod graphics;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    // the kernel is mapped into the higher half of the virtual address space.
    config.mappings.dynamic_range_start = Some(0xFFFF_8000_0000_0000);
    config.mappings.page_table_recursive = Some(Mapping::Dynamic);

    config
};

#[cfg(not(test))]
bootloader_api::entry_point!(arch_init, config = &BOOTLOADER_CONFIG);

pub fn arch_init(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    println!("boot_info: {:#?}", boot_info);
    print!("Loading IDT...");
    cpu::interrupts::idt::init_idt();
    crate::kmain();
}
