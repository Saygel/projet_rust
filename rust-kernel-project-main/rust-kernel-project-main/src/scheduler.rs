#![no_std]

use crate::process::{Process, ProcessState};
use heapless::Vec;

pub struct PriorityRoundRobinScheduler {
    process_queue: Vec<Process, 8>, // File de processus, max 8
}

impl PriorityRoundRobinScheduler {
    pub fn new() -> Self {
        PriorityRoundRobinScheduler {
            process_queue: Vec::new(),
        }
    }

    pub fn add_process(&mut self, process: Process) {
        self.process_queue.push(process).ok(); // Ajoute un processus s'il reste de la place
    }

    pub fn run(&mut self) {
        loop {
            if self.process_queue.is_empty() {
                break; // Pas de processus à exécuter
            }

            let highest_priority_pid = self.find_highest_priority();
            
            if let Some(index) = highest_priority_pid {
                let mut process = &mut self.process_queue[index];

                if process.state == ProcessState::Ready {
                    process.run();
                    
                    // Simuler un quantum de temps avant de changer de processus
                    self.handle_time_slice(&mut process);

                    // Replacer le processus dans la file
                    self.process_queue.swap_remove(index);
                    self.process_queue.push(process.clone()).ok();
                }
            }
        }
    }

    fn find_highest_priority(&self) -> Option<usize> {
        let mut highest_priority: u8 = 255; // Plus petite valeur = plus haute priorité
        let mut index: Option<usize> = None;

        for (i, process) in self.process_queue.iter().enumerate() {
            if process.state == ProcessState::Ready && process.priority < highest_priority {
                highest_priority = process.priority;
                index = Some(i);
            }
        }
        index
    }

    fn handle_time_slice(&self, process: &mut Process) {
        // Simuler une tranche de temps : après l'exécution, on passe en état "Waiting"
        process.state = ProcessState::Waiting;
    }

    pub fn stop_all(&mut self) {
        for process in self.process_queue.iter_mut() {
            process.stop();
        }
    }
}