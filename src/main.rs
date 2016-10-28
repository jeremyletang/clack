#![feature(proc_macro, custom_derive)]


#[macro_use] extern crate chan;
extern crate chan_signal;
extern crate env_logger;
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate ws;

use chan_signal::{Signal, notify};
use std::thread;

mod conf;
mod slack;

fn start_ws(s: &str) {
    // Connect to the url and call the closure
    ws::connect(s, |out| {
        
        // Queue a message to be sent when the WebSocket is open
        // if let Err(_) = out.send("Hello WebSocket") {
        //    println!("Websocket couldn't queue an initial message.")
        //} else {
        //    println!("Client sent message 'Hello WebSocket'. ")
        //}

        // The handler needs to take ownership of out, so we use move
        move |msg| {

            // Handle messages received on this connection
            println!("Client got message '{}'. ", msg);
            match msg {
                ws::Message::Text(s) => slack::events::_match(s),
                _ => println!("unsupported binary format"),
            };
            // Close the connection
           // out.close(ws::CloseCode::Normal)
            Ok(())
        }
    });
    println!("exited thread");
}

fn main() {
    env_logger::init().unwrap();

    let c = match conf::load() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return
        }
    };


    thread::spawn(move || {
        match slack::rtm::start::call(&*c.tokens.first().unwrap()) {
            Ok(s) => start_ws(&*s.url),
            Err(e) => println!("{}", e)
        }
    });

    let signal = notify(&[Signal::INT, Signal::QUIT, Signal::KILL]);
    signal.recv();
}
