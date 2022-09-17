use rdev::{listen, Event, EventType::*};
use std::{thread, sync::mpsc::Sender};

pub fn event_thread(evt_schan: Sender<Event>) {
    // event listener
    thread::spawn(move || {
        listen(move |event: Event| {
            match event.event_type {
                KeyRelease(_) |
                ButtonRelease(_) => {},
                _ => {
                    evt_schan
                    .send(event)
                    .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                }
            }
        }).expect("Could not listen");
    });
}