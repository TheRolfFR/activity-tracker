
use std::{thread, time::Duration};
use std::sync::mpsc::channel;

use iced::{task::{Never, Sipper, sipper}, Subscription};
use activity_tracker_lib::threads::*;
use activity_tracker_lib::data::payload::Payload;

use crate::state::MainMessage;

fn data_stream() -> impl Sipper<Never, MainMessage> {

    let (evt_schan, evt_rchan) = channel::<DataMessage>();
    let (payload_schan, payload_rchan) = channel::<Payload>();

    sipper(async move |mut output| {
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        event_thread(evt_schan);

        thread::spawn(move || -> () {
            data_thread(evt_rchan, payload_schan);
        });

        loop {
            interval.tick().await;

            if let Ok(mut payload) =  payload_rchan.try_recv() {
                payload.version = format!("{}-{}-{}", env!("GIT_VERSION"), env!("GIT_HASH"), env!("GIT_DATE"));
                output.send(MainMessage::Payload(payload)).await;
            }
        }
    })
}

pub fn data_subscription() -> Subscription<MainMessage> {
    Subscription::run(data_stream)
}
