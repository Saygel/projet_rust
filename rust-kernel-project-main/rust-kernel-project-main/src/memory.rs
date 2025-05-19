#![no_std]

use core::ptr::{self, NonNull};

pub struct BuddyAllocator {
    start: NonNull<u8>,
    block_size: usize,
    total_blocks: usize,
    free_list: Vec<Option<NonNull<u8>>, 16>, // Liste des blocs libres (max 16 niveaux)
}

impl BuddyAllocator {
    /// Initialise un nouvel allocateur Buddy System
    pub unsafe fn new(start: NonNull<u8>, block_size: usize, total_blocks: usize) -> Self {
        let mut allocator = BuddyAllocator {
            start,
            block_size,
            total_blocks,
            free_list: Vec::new(),
        };

        allocator.init();
        allocator
    }

    /// Initialise la mémoire et crée la liste des blocs libres
    unsafe fn init(&mut self) {
        for i in 0..self.total_blocks {
            let block = self.start.as_ptr().add(i * self.block_size);
            self.free_list.push(Some(NonNull::new_unchecked(block))).ok();
        }
    }

    /// Alloue un bloc de mémoire et retourne son pointeur
    pub unsafe fn allocate(&mut self) -> Option<NonNull<u8>> {
        for i in 0..self.free_list.len() {
            if let Some(block) = self.free_list[i].take() {
                return Some(block);
            }
        }
        None // Pas de mémoire disponible
    }

    /// Libère un bloc de mémoire et le remet dans la liste libre
    pub unsafe fn deallocate(&mut self, block: NonNull<u8>) {
        for i in 0..self.free_list.len() {
            if self.free_list[i].is_none() {
                self.free_list[i] = Some(block);
                return;
            }
        }
    }

    /// Fonction de traduction d’adresse virtuelle en adresse physique (simple simulation)
    pub fn virt_to_phys(virt_addr: *const u8) -> *const u8 {
        virt_addr
    }
}