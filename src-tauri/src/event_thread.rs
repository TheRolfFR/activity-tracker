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
                    .ok();
                }
            }
        }).expect("Could not listen");
    });
}