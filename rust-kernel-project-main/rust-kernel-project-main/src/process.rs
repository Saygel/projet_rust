#![no_std]

use heapless::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use core::ptr::NonNull;

static NEXT_PID: AtomicU32 = AtomicU32::new(1);

pub struct Process {
    pub pid: u32,                    // Identifiant unique du processus
    pub priority: u8,                 // Niveau de priorité (0 = élevé, 255 = faible)
    pub state: ProcessState,          // État du processus
    pub memory: Option<NonNull<u8>>,  // Allocation mémoire associée
    pub message_queue: Vec<Message, 4>, // File d'attente des messages IPC
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessState {
    Ready,   
    Running, 
    Waiting, 
    Stopped, 
}

pub struct Message {
    pub sender: u32,
    pub data: [u8; 16], // Message limité à 16 octets
}

impl Process {
    pub fn new(priority: u8) -> Self {
        Process {
            pid: NEXT_PID.fetch_add(1, Ordering::SeqCst),
            priority,
            state: ProcessState::Ready,
            memory: None,
            message_queue: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        self.state = ProcessState::Running;
    }

    pub fn stop(&mut self) {
        self.state = ProcessState::Stopped;
    }

    pub fn send_message(&mut self, msg: Message) {
        let _ = self.message_queue.push(msg); // Ajoute un message dans la file d'attente
    }

    pub fn receive_message(&mut self) -> Option<Message> {
        self.message_queue.pop() // Récupère un message si disponible
    }
}

// Initialisation des processus avec IPC
pub fn init_processes() -> Vec<Process, 8> {
    let mut processes = Vec::new();
    processes.push(Process::new(1)).ok(); // Processus avec priorité 1
    processes.push(Process::new(3)).ok(); // Processus avec priorité 3
    processes
}