#![no_std]
#![no_main]

use crate::filesystem::FAT32;
use core::ptr::NonNull;

#[test_case]
fn test_fs_allocation() {
    let start_memory = 0x2000 as *mut u8;
    let block_size = 512;
    let block_count = 32;

    unsafe {
        let mut fs = FAT32::new(start_memory, block_size, block_count);
        let block = fs.allocate_block();
        assert!(block.is_some()); // Vérifie que l'allocation d'un bloc réussit
    }
}

#[test_case]
fn test_fs_write_read() {
    let start_memory = 0x2000 as *mut u8;
    let block_size = 512;
    let block_count = 32;

    unsafe {
        let mut fs = FAT32::new(start_memory, block_size, block_count);
        let block = fs.allocate_block().unwrap();
        let data = b"Test Data";
        fs.write_file(block, data).unwrap();
        let read_data = fs.read_file(block, data.len()).unwrap();
        assert_eq!(read_data, data);
    }
}