/// Asynchronous controller client (DEALER)
/// 
/// This binary spawns multiple clients that run in a single process to easily
/// start and stop the all the clients. Each client task acts as its own
/// separate entity.

use std::{thread, time};

use iot_comm::core::Controller;


/// A controller client task
/// 
/// Pushes the controller's sensor data to the server every 5 seconds via TCP,
/// and prints any messages it receives from the server.
fn controller_client_task() {
    let context = zmq::Context::new();
    let client = context.socket(zmq::DEALER).unwrap();
    let mut controller = Controller::new();

    client.set_identity(controller.id.as_bytes())
        .expect("failed setting client id");
    client.connect("tcp://localhost:5570")
        .expect("failed connecting client");

    // client task running loop
    loop {
        if client.poll(zmq::POLLIN, 10).expect("client failed polling") > 0 {
            let msg = client.recv_string(0)
                .expect("client failed receivng response");
            
            println!("controller {}: {}", controller.id, &msg.unwrap());
        }

        client.send(controller.sensor_data(), 0)
            .expect("client failed sending request");
        
        thread::sleep(time::Duration::from_secs(5));
    }
}

/// Runs a number of clients.
/// 
/// Spawns a new thread for each controller client.
fn main() {
    for i in 0..1000 {
        let builder = thread::Builder::new()
            .name(format!("controller {}", &i));
        
        builder.spawn(controller_client_task)
            .expect("Could not spawn thread");
    }
    loop {}
}
