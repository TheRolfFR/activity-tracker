use std::{sync::mpsc::{Receiver, Sender}, time::Instant};

use rdev::{Event, EventType::{ButtonPress, KeyPress}};

use crate::{data::{ONE_SECOND, ONE_MINUTE, MeasureCount}, day_record_file::DayRecordFile};

pub fn day_thread(evt_rchan: Receiver<Event>, evt_schan: Sender<()>) -> ! {
    let mut click_count: MeasureCount = 0;
    let mut input_count: MeasureCount = 0;

    let mut last_second = Instant::now();
    let mut last_minute = Instant::now();
    loop {
        let res_event = evt_rchan.recv_timeout(ONE_SECOND);

        let now = Instant::now();

        // diverge and converge
        let one_sec = if let Ok(event) = res_event {
            match event.event_type {
                KeyPress(_) => { input_count = input_count.saturating_add(1); }
                ButtonPress(_) => { click_count = click_count.saturating_add(1); },
                _ => unreachable!()
            };

            now.duration_since(last_second) >= ONE_SECOND
        } else {
            // one sec for sure
            true
        };

        let one_minute = now.duration_since(last_minute) >= ONE_MINUTE;

        if one_sec {
            evt_schan.send(()).expect("Couldn't send display update");
            last_second = now;
        }

        if one_minute {
            DayRecordFile::update(click_count, input_count);

            click_count = 0;
            input_count = 0;

            last_minute = now;
        }
    }
}