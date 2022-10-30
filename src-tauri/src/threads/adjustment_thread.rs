use std::sync::mpsc::{Receiver, Sender};

use log::error;

use super::DataMessage;

pub fn adjustment_thread(adj_rcv: Receiver<i32>, evt_send: Sender<DataMessage>) {
    loop {
        match adj_rcv.recv() {
            Ok(adj) => {
                if let Err(err_send) = evt_send.send(DataMessage::Adjustement(adj)) {
                    error!("Failed to send async adjustment: {err_send}");
                }
            },
            Err(err) => {
                error!("Failed to received adjustment in thread: {err}");
            }
        };
    }
}