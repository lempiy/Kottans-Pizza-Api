use redis::{Commands, Connection, PubSub};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

pub struct PubSubEvent {
    pub channel: String,
    pub message: String,
}

enum Action {
    #[allow(dead_code)] Subscribe(String),
    #[allow(dead_code)] Unsubscribe(String),
    Send(PubSubEvent),
}

pub struct Manager {
    send_channel: Sender<Action>,
    map: Arc<Mutex<HashMap<String, Vec<Sender<String>>>>>,
}

impl Manager {
    pub fn new(ps: PubSub, conn: Arc<Mutex<Connection>>) -> Manager {
        let pub_sub = Arc::new(Mutex::new(ps));
        let (tx, rx) = channel();
        let sub_pub_sub = pub_sub.clone();
        let map: Arc<Mutex<HashMap<String, Vec<Sender<String>>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let read_map = map.clone();
        thread::spawn(move || {
            loop {
                let action: Action = rx.recv().expect("Unable to receive from channel");
                match action {
                    Action::Send(event) => {
                        let cn = conn.lock().unwrap();
                        let _result =
                            cn.publish::<String, String, String>(event.channel, event.message);
                    }
                    Action::Subscribe(channel_name) => {
                        let mut pubsub = sub_pub_sub.lock().unwrap();
                        pubsub
                            .subscribe::<&str>(&channel_name)
                            .expect(&("Unable to subscribe to ".to_string() + &channel_name));
                    }
                    Action::Unsubscribe(channel_name) => {
                        let mut pubsub = sub_pub_sub.lock().unwrap();
                        pubsub
                            .unsubscribe::<&str>(&channel_name)
                            .expect(&("Unable to unsubscribe to ".to_string() + &channel_name));
                    }
                }
            }
        });
        thread::spawn(move || {
            loop {
                let pubsub = pub_sub.lock().unwrap();
                let msg = pubsub.get_message().unwrap();
                let message: String = msg.get_payload().unwrap();
                let channel = msg.get_channel_name();
                let map = read_map.lock().unwrap();
                if let Some(subs) = map.get(channel) {
                    for tx in subs.iter() {
                        let _ = tx.send(message.clone());
                    }
                };
            }
        });
        Manager {
            send_channel: tx,
            map,
        }
    }

    pub fn send(&self, event: PubSubEvent) {
        let _ = self.send_channel.send(Action::Send(event));
    }

    #[allow(dead_code)]
    pub fn subscribe(&self, channel_name: String) -> Receiver<String> {
        let (tx, rx) = channel();
        let mut map = self.map.lock().unwrap();
        if let Some(subs) = map.get_mut(&channel_name) {
            subs.push(tx);
            return rx;
        };
        let _ = self.send_channel
            .send(Action::Subscribe(channel_name.clone()));
        let subs = vec![tx];
        map.insert(channel_name, subs);
        rx
    }
}
