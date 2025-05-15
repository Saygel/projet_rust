#![no_std]
#![no_main]
#![feature(asm)]

use core::ptr;



impl Heap {
    pub const fn new() -> Self {
        Heap {
            heap_debut: ptr::null_mut(),
            heap_fin: ptr::null_mut(),
        }
    }

    pub unsafe fn creation(&mut self) -> bool {
        let mut limite_actuelle: usize;


        asm!(
            "mov rax, 12",               // Code de l'appel brk (syscall number pour brk)
            "mov rdi, 0",                // RDI = 0 pour obtenir la taille actuelle du brk
            "syscall",                   // Exécuter le syscall
            "mov {0}, rax",              // Récupérer l'adresse dans limite_actuelle
            out(reg) limite_actuelle     // Le résultat de l'appel dans limite_actuelle
        );

        if limite_actuelle == 0 {
            return false; 
        }

        let mut resultat_limite: usize;

        asm!(
            "mov rax, {0}",              // Charger la limite actuelle dans rax
            "add rax, 0x3000",           // Ajouter 0x3000 à la limite actuelle
            "mov rdi, rax",              // Charger la nouvelle adresse (limite étendue) dans rdi
            "mov rax, 12",               // Code de l'appel brk
            "syscall",                   // Exécuter le syscall
            "mov {1}, rax",              // Récupérer la nouvelle limite dans resultat_limite
            in(reg) limite_actuelle, out(reg) resultat_limite
        );

        if resultat_limite == limite_actuelle {
            return false; *
        }

        self.heap_debut = limite_actuelle as *mut u8;
        self.heap_fin = resultat_limite as *mut u8; 

        true
    }

    pub unsafe fn extension(&mut self) -> bool {
        let mut resultat_limite: usize;

        asm!(
            "mov rax, {0}",              // Charger l'ancienne limite de fin dans rax
            "add rax, 0x3000",           // Ajouter 0x3000 pour étendre
            "mov rdi, rax",              // Charger la nouvelle adresse (limite étendue) dans rdi
            "mov rax, 12",               // Code de l'appel brk
            "syscall",                   // Exécuter le syscall
            "mov {1}, rax",              // Récupérer la nouvelle limite dans resultat_limite
            in(reg) self.heap_fin, out(reg) resultat_limite
        );

        if resultat_limite == self.heap_fin as usize {
            return false; 
        }

        self.heap_fin = resultat_limite as *mut u8;

        true
    }
}

unsafe fn configuration(heap: &mut Heap) -> bool {
    if !heap.creation() {
        return false;  
    }

    if !heap.extension() {
        return false; 
    }

    true
}
