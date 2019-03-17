extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();
    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            let client = request.accept().unwrap();
            let ip = client.peer_addr().unwrap();
            println!("Connection from {}", ip);
            let (mut receiver, mut sender) = client.split().unwrap();
            for message_result in receiver.incoming_messages() {
                match message_result {
                    Err(_) => {
                        println!("Client {} disconnected unexpectedly", ip);
                        return;
                    }
                    _ => (),
                }
                let message = message_result.unwrap();
                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        sender.send_message(&OwnedMessage::Pong(ping)).unwrap()
                    }
                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }
}
