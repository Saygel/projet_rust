#![no_std]
#![no_main]

use crate::memory::{BuddyAllocator};
use core::ptr::NonNull;

#[test_case]
fn test_memory_allocation() {
    let start_memory = 0x1000 as *mut u8;
    let block_size = 128;
    let block_count = 16;

    unsafe {
        let mut allocator = BuddyAllocator::new(NonNull::new(start_memory).unwrap(), block_size, block_count);
        let block = allocator.allocate();
        assert!(block.is_some()); 
    }
}

#[test_case]
fn test_memory_deallocation() {
    let start_memory = 0x1000 as *mut u8;
    let block_size = 128;
    let block_count = 16;

    unsafe {
        let mut allocator = BuddyAllocator::new(NonNull::new(start_memory).unwrap(), block_size, block_count);
        let block = allocator.allocate().unwrap();
        allocator.deallocate(block);
        let new_block = allocator.allocate();
        assert!(new_block.is_some()); // Vérifie que la mémoire libérée peut être réutilisée
    }
}