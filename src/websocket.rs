use crate::model::Hello;
use std::{env, io::ErrorKind, net::TcpStream, time::{Instant, SystemTime, UNIX_EPOCH}};
use log::debug;
use pyo3::{Python, types::PyDict};
use serde_json::json;
use tungstenite::{Message, WebSocket, connect, stream::MaybeTlsStream};

use crate::model::{MessageCreate, OpCode, RawReceivedPayload};

pub struct WebSocketHandler {
    token: String,
    ws_url: String,
}
pub struct DiscordProtocol {
    pub ws: WebSocket<MaybeTlsStream<TcpStream>>,
    token: String,
    pub heartbeat_interval: u64,
    pub last_heartbeat: Instant,
    pub recent_acks: std::collections::VecDeque<f64>,
}

impl WebSocketHandler {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
            ws_url: "wss://gateway.discord.gg/?v=9&encoding=json".to_owned()
        }
    }

    pub fn connect(&self) -> DiscordProtocol {
        let (socket, _) = connect(self.ws_url.to_owned()).unwrap();
        DiscordProtocol {
            ws: socket,
            token: self.token.to_owned(),
            heartbeat_interval: u64::MAX,
            last_heartbeat: Instant::now(),
            recent_acks: std::collections::VecDeque::with_capacity(20)
        }
    }
}



impl DiscordProtocol {
    pub fn recv(&mut self, py: Python, listeners: &PyDict) -> Option<()> {
        if self.last_heartbeat.elapsed().as_millis() as u64 >= self.heartbeat_interval {
            self.heartbeat();
        }

        let msg = {
            match self.ws.read_message() {
                Err(tungstenite::Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut  => return None,
                Err(_) => return None,
                Ok(msg) => msg,
            }
        };

        match msg {
            Message::Text(text) => {
                let payload: RawReceivedPayload = serde_json::from_str(&text).unwrap();

                if payload.op != OpCode::HEARTBEAT_ACK {
                    debug!("Received payload: {:?}", payload);
                }

                match payload.op {
                    OpCode::HELLO => {
                        self.identify();
                        let payload: Hello = serde_json::from_str(payload.d.get()).unwrap();
                        let interval = payload.heartbeat_interval as u64;
                        self.heartbeat_interval = interval.min(5000);
                        self.last_heartbeat = Instant::now();
                        self.heartbeat();
                        if let Some(item) = listeners.get_item("Ready") {
                            let _ = item.call0();
                        }
                        Some(())
                    },
                    OpCode::HEARTBEAT_ACK => {
                        let now = Instant::now();
                        let delta = now.duration_since(self.last_heartbeat);
                        if self.recent_acks.len() == 20 {
                            self.heartbeat();
                            self.recent_acks.pop_front();
                        }
                        self.recent_acks.push_back(delta.as_secs_f64());
                        Some(())
                    },
                    OpCode::INVALID_SESSION => return None,
                    OpCode::DISPATCH => {
                        match payload.t {
                            Some("MESSAGE_CREATE") => {
                                if let Some(item) = listeners.get_item("MessageCreate") {
                                    let message: MessageCreate = serde_json::from_str(&payload.d.get()).unwrap();
                                    let obj = pythonize::pythonize(py, &message).unwrap();
                                    let _ = item.call1((obj,));
                                }
                            },
                            Some(_) => (),
                            None => (),
                        }
                        Some(())
                    },
                    _ => Some(()),
                }
            },
            Message::Close(_) => {
                println!("Received close message: {:?}", msg);
                self.ws.close(None).unwrap();

                None
            },
            _ => None,
        }
    }

    pub fn identify(&mut self) {
        self.ws.write_message(Message::Text(json!({
            "op": OpCode::IDENTIFY,
            "d": {
              "token": self.token,
              "intents": 513,
              "properties": {
                "$os": env::consts::OS,
                "$browser": "Concord (https://github.com/akiacode/concord)",
                "$device": "Concord (https://github.com/akiacode/concord)"
            }
        }
        }).to_string())).unwrap();
    }

    pub fn heartbeat(&mut self) {
        println!("Sending heartbeat, last_heartbeat: {}, heartbeat_interval: {}", self.last_heartbeat.elapsed().as_millis(), self.heartbeat_interval);

        self.ws.write_message(Message::Text(json!({
            "op": OpCode::HEARTBEAT,
            "d": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
        }).to_string())).unwrap();
        self.last_heartbeat = Instant::now();
    }
}