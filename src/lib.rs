use dashmap::DashMap;
use std::sync::{Arc};
use futures_util::{SinkExt, StreamExt};
use warp::ws::{Message, WebSocket};

pub type Core = Arc<DashMap<String, String>>;

pub async fn ws_process(websocket: WebSocket, core: Core) {
    // Split the WebSocket into tx and rx.
    let (mut tx, mut rx) = websocket.split();

    // Continuously loop for Items by calling next() on the rx Stream.
    while let Some(item) = rx.next().await {
        match item {
            Ok(message) => {
                // Collect the Query Args.
                if let Ok(message_string) = message.to_str() {
                    let query = message_string.splitn(5, " ").collect();

                    // Process the Query.
                    let process_value = core_process(core.clone(), query);

                    // Send the Value back to the Client.
                    tx.send(Message::text(process_value)).await.unwrap();
                } else  {
                    // If any error, print back to the console.
                    eprintln!("Error occurred while reading the Query");
                }
            }
            Err(error) => {
                // If any error, print back to the console.
                eprintln!("{}", error);
                break;
            }
        };
    }
}

pub fn http_process(core: Core, query: String) -> String {
    let query_vec: Vec<&str> = query.splitn(5,  " ").collect();

    core_process(core.clone(), query_vec)
}

// Private Core Process Fn
fn core_process(core: Core, query: Vec<&str>) -> String {
    match query[0] {
        "GET" => {
            if let Some(dash_value) = core.get(query[1]) {
                dash_value.value().clone()
            } else {
                String::from("Nil")
            }
        }
        "SET" => {
            if let Some(..) = core.insert(String::from(query[1]), String::from(query[2])) {
                String::from("Similar key exists")
            } else {
                String::from("Inserted")
            }
        }
        "DEL" => {
            if let Some(..) = core.remove(query[1]) {
                String::from("Removed")
            } else {
                String::from("Failed to remove!")
            }
        }
        &_ => {
            String::from("Command not implemented!")
        }
    }
}
