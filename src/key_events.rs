
use std::{thread, time::Duration};
use std::sync::mpsc::channel;

use activity_tracker_lib::data::state::ActivityState;
use iced::{task::{Never, Sipper, sipper}, Subscription};
use activity_tracker_lib::threads::*;

use crate::state::MainMessage;

fn data_stream() -> impl Sipper<Never, MainMessage> {

    let (evt_schan, evt_rchan) = channel::<DataMessage>();
    let (state_schan, state_rchan) = channel::<ActivityState>();

    sipper(async move |mut output| {
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        event_thread(evt_schan);

        thread::spawn(move || -> () {
            data_thread(evt_rchan, state_schan);
        });

        loop {
            interval.tick().await;

            if let Ok(act_state) =  state_rchan.try_recv() {
                output.send(MainMessage::Payload(act_state)).await;
            }
        }
    })
}

pub fn data_subscription() -> Subscription<MainMessage> {
    Subscription::run(data_stream)
}
