/// Asynchronous server (ROUTER)
/// 
/// This binary runs a server on a TCP socket that proxies messages to number
/// worker tasks handle the messages and perform some action.

use std::thread;

use iot_comm::core::Sensor;


/// The main server task.
/// 
/// This acts as a proxy layer that passes clients requests received over TCP
/// to a number of sever workers tasks via IPC.
fn server_task() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    frontend
        .bind("tcp://*:5570")
        .expect("server failed binding frontend");
    let backend = context.socket(zmq::DEALER).unwrap();
    backend
        .bind("inproc://backend")
        .expect("server failed binding backend");
    for _ in 0..5 {
        let ctx = context.clone();
        thread::spawn(move || server_worker(&ctx));
    }
    zmq::proxy(&frontend, &backend).expect("server failed proxying");
}

/// A server worker.
/// 
/// Recevives messages from passed from the main server task and handles them
/// in the appropriate manner.
/// 
/// - `context` - the server backend context
fn server_worker(context: &zmq::Context) {
    let worker = context.socket(zmq::DEALER).unwrap();
    worker
        .connect("inproc://backend")
        .expect("worker failed to connect to backend");

    loop {
        let identity = worker
            .recv_string(0)
            .expect("worker failed receiving identity")
            .unwrap();
        let message = worker
            .recv_bytes(0)
            .expect("worker failed receiving message");

        let mut log = identity.clone();
        log.push_str(":\n");
        for (i, chunk) in message.chunks(4).enumerate() {
            log.push_str(format!("  sensor {}: {}\n", i, Sensor::from(chunk)).as_str());
        }
        println!("{}", log);

        worker
            .send(&identity, zmq::SNDMORE)
            .expect("worker failed sending identity");
        worker
            .send("R", 0)
            .expect("worker failed sending message");
    }
}

/// Runs the server.
fn main() {
    server_task();
    loop {}
}
