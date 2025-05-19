# Rust Kernel Project 🚀  
Petit noyau minimal en Rust, avec un **ordonnanceur Priorité-Round Robin**, un **allocateur de mémoire Buddy System** et un **système de fichiers FAT32**. 

## ⚙️ Fonctionnalités principales
- **Ordonnanceur Priorité-Round Robin** : Donne plus de temps CPU aux processus prioritaires.  
- **Allocateur Buddy System** : Gestion mémoire efficace, réduit la fragmentation.  
- **FAT32 avec cache** : Système de fichiers rapide, lecture/écriture optimisées.  
- **IPC (Inter-Process Communication)** : Les processus peuvent s'envoyer des messages.  
- **Tests unitaires** : Vérifient le bon fonctionnement des modules.

## 📖 Ressources utiles  
- [Writing an OS in Rust](https://os.phil-opp.com/) – Blog de Phil Oppmann sur la création d'un OS en Rust.  
- [Rust for Embedded Systems](https://docs.rust-embedded.org/book/) – Pour comprendre `no_std` et la gestion mémoire en Rust.  
- [FAT32 File System Explained](https://www.pjrc.com/tech/8051/ide/fat32.html) – Explication du fonctionnement de FAT32.  
- [Buddy Memory Allocation](https://en.wikipedia.org/wiki/Buddy_memory_allocation) – Concept du Buddy System pour la gestion mémoire.  
