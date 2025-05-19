#![no_std]

use core::ptr::NonNull;
use core::slice;
use heapless::Vec; // Utilisation de Vec pour l'environnement no_std

pub struct FAT32 {
    start: NonNull<u8>,         // Adresse de début de la mémoire du système de fichiers
    block_size: usize,         // Taille de chaque bloc dans le système de fichiers
    block_count: usize,        // Nombre total de blocs disponibles
    fat_table: NonNull<u32>,   // Table d'allocation de fichiers (FAT)
    root_dir: NonNull<u8>,     // Répertoire racine
    cache: Vec<[u8; 512], 8>,  // Cache de blocs pour accélérer les accès
}

impl FAT32 {
    /// Crée un nouveau système de fichiers FAT32 avec cache
    pub unsafe fn new(start: *mut u8, block_size: usize, block_count: usize) -> Self {
        let start_nn = NonNull::new(start).expect("Start address cannot be null");

        // Allocation de la table FAT et du répertoire racine
        let fat_table = NonNull::new(start.add(1024) as *mut u32).expect("FAT Table allocation failed");
        let root_dir = NonNull::new(start.add(2048) as *mut u8).expect("Root Directory allocation failed");

        FAT32 {
            start: start_nn,
            block_size,
            block_count,
            fat_table,
            root_dir,
            cache: Vec::new(),
        }
    }

    /// Alloue un bloc de données pour un fichier
    pub unsafe fn allocate_block(&mut self) -> Option<usize> {
        let fat_slice = slice::from_raw_parts_mut(self.fat_table.as_ptr(), self.block_count);
        for (i, entry) in fat_slice.iter_mut().enumerate() {
            if *entry == 0 { // Bloc libre
                *entry = 1;  // Marque le bloc comme utilisé
                return Some(i);
            }
        }
        None
    }

    /// Libère un bloc de données
    pub unsafe fn deallocate_block(&mut self, block_idx: usize) {
        let fat_slice = slice::from_raw_parts_mut(self.fat_table.as_ptr(), self.block_count);
        fat_slice[block_idx] = 0; // Marque le bloc comme libre
    }

    /// Écrit des données dans un fichier avec mise en cache
    pub unsafe fn write_file(&mut self, block_idx: usize, data: &[u8]) -> Result<(), &'static str> {
        if block_idx >= self.block_count {
            return Err("Invalid block index");
        }

        // Stocker dans le cache
        if self.cache.len() < 8 {
            let mut cached_block = [0u8; 512];
            cached_block[..data.len()].copy_from_slice(data);
            self.cache.push(cached_block).ok();
        }

        // Écrire en mémoire
        let block_ptr = self.start.as_ptr().add(block_idx * self.block_size);
        let slice = slice::from_raw_parts_mut(block_ptr, data.len());
        slice.copy_from_slice(data);
        Ok(())
    }

    /// Lit un fichier avec support du cache
    pub unsafe fn read_file(&self, block_idx: usize, size: usize) -> Result<&[u8], &'static str> {
        if block_idx >= self.block_count {
            return Err("Invalid block index");
        }

        // Vérifier si le bloc est en cache
        if let Some(cached_block) = self.cache.get(block_idx % 8) {
            return Ok(&cached_block[..size]);
        }

        // Lire depuis la mémoire
        let block_ptr = self.start.as_ptr().add(block_idx * self.block_size);
        let slice = slice::from_raw_parts(block_ptr, size);
        Ok(slice)
    }

    /// Liste les fichiers du répertoire racine (simulation)
    pub unsafe fn list_root_dir(&self) -> Vec<&[u8], 8> {
        let root_dir_slice = slice::from_raw_parts(self.root_dir.as_ptr(), self.block_count);
        let mut files = Vec::new();

        for i in 0..self.block_count {
            if root_dir_slice[i] != 0 {
                files.push(slice::from_raw_parts(self.root_dir.as_ptr().add(i * self.block_size), self.block_size));
            }
        }
        files
    }
}