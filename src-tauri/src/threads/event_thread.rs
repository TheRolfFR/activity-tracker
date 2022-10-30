use rdev::{listen, Event, EventType::*};
use std::{thread, sync::mpsc::Sender};
use log::error;

use crate::{records::Record, throttle::Throttle, data::ONE_SECOND};

use super::DataMessage;

pub fn event_thread(evt_schan: Sender<DataMessage>) {
    let mut record = Record::default();
    let mut evt_throttle = Throttle::new(ONE_SECOND, 1);
    // event listener
    thread::spawn(move || {
        if let Err(err_listen) = listen(move |event: Event| {
            let modified = match event.event_type {
                KeyRelease(_) |
                ButtonRelease(_) |
                MouseMove { x: _, y: _ } |
                Wheel { delta_x: _, delta_y: _ } => false,
                KeyPress(_) => {
                    record.inputs = record.inputs.saturating_add(1);
                    true
                },
                ButtonPress(_) => {
                    record.clicks = record.clicks.saturating_add(1);
                    true
                }
            };

            if modified && evt_throttle.update() {
                if let Err(err_send) = evt_schan.send(DataMessage::EventRecord(record)) {
                    error!("Failed to send record in data thread: {err_send}");
                }
                // reset
                record = Record::default();
            }
        }) {
            error!("Failed to listen in data thread: {:?}", err_listen);
        }
    });
}