//!
//! Ce projet implémente un noyau simplifié avec gestion avancée des processus et de la mémoire.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod process;      // Gestion des processus avec IPC
pub mod scheduler;    // Ordonnanceur Priorité-Round Robin
pub mod memory;       // Allocateur Buddy System
pub mod filesystem;   // Système de fichiers FAT32 avec cache

use core::ptr::NonNull;
use memory::{virt_to_phys, BuddyAllocator};
use heapless::Vec;  // Utilisation de Vec dans no_std

// Fonction pour initialiser l'allocation mémoire
pub fn init_memory_allocator(start: *mut u8, block_size: usize, block_count: usize) -> BuddyAllocator {
    unsafe {
        let start_nn = NonNull::new(start).expect("Start address cannot be null");
        BuddyAllocator::new(start_nn, block_size, block_count)
    }
}

// Fonction pour initialiser le système de fichiers FAT32 avec cache
pub fn init_filesystem(start: *mut u8, block_size: usize, block_count: usize) -> filesystem::FAT32 {
    unsafe {
        filesystem::FAT32::new(start, block_size, block_count)
    }
}

// Fonction pour initialiser l'ordonnanceur Priorité-Round Robin
pub fn init_scheduler() -> scheduler::PriorityRoundRobinScheduler {
    scheduler::PriorityRoundRobinScheduler::new()
}

// Fonction pour initialiser les processus avec gestion d’IPC
pub fn init_processes() -> Vec<process::Process, 8> {
    let mut processes = Vec::new();
    processes.push(process::Process::new(1, 1)).ok(); // Processus avec priorité 1
    processes.push(process::Process::new(2, 3)).ok(); // Processus avec priorité 3
    processes
}

// Fonction principale du noyau
pub fn kernel_main() {
    let start_memory = 0x1000 as *mut u8;
    let block_size = 128;
    let block_count = 1024;
    let _allocator = init_memory_allocator(start_memory, block_size, block_count);

    let _filesystem = init_filesystem(start_memory, block_size, block_count);
    let mut _scheduler = init_scheduler();
    let mut _processes = init_processes();

    loop {
        _scheduler.run(); // Exécution continue de l'ordonnanceur
    }
}