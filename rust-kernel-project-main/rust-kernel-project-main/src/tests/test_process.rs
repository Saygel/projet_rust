#![no_std]
#![no_main]

use crate::process::{Process, ProcessState, Message};

#[test_case]
fn test_process_creation() {
    let process = Process::new(1);
    assert_eq!(process.state, ProcessState::Ready);
    assert_eq!(process.priority, 1);
}

#[test_case]
fn test_process_run_stop() {
    let mut process = Process::new(2);
    process.run();
    assert_eq!(process.state, ProcessState::Running);
    process.stop();
    assert_eq!(process.state, ProcessState::Stopped);
}

#[test_case]
fn test_process_ipc() {
    let mut process = Process::new(3);
    let message = Message { sender: 1, data: *b"Hello World!" };
    process.send_message(message);
    let received = process.receive_message().unwrap();
    assert_eq!(received.data, *b"Hello World!");
}