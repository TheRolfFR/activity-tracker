use std::sync::mpsc::{Receiver, Sender};

use log::error;

use crate::data::records::DayRecord;
use crate::data::state::ActivityState;
use crate::data::{ONE_MINUTE, ONE_SECOND, records::Record};
use crate::{storage::DayRecordFile, throttle::Throttle};

pub enum DataMessage {
    EventRecord(Record),
    Adjustement(i32)
}

pub fn data_thread(evt_rchan: Receiver<DataMessage>, evt_schan: Sender<ActivityState>) -> ! {
    let mut state = ActivityState::default();

    // once per minute
    let mut file_throttle = Throttle::new(ONE_MINUTE, 1);
    let mut ui_throttle = Throttle::new(ONE_SECOND, 1);
    let min_timeout = ONE_MINUTE.min(ONE_SECOND);

    loop {
        let res_message = evt_rchan.recv_timeout(min_timeout);

        // diverge and converge
        if let Ok(message) = res_message {
            match message {
                DataMessage::EventRecord(record) => {
                    state.update_today(record);
                },
                DataMessage::Adjustement(adjusted) => {
                    state.update_adjusted(adjusted);
                }
            }
        }

        if ui_throttle.update() {
            if let Err(err) = evt_schan.send(state.clone()) {
                error!("Failed to send payload from data thread: {}", err);
            }
        }

        if file_throttle.update() && let Some(today) = state.today().as_ref()
            .map(|s| DayRecord::from_day_state(ActivityState::date(), s)) {
            DayRecordFile::save(&today);
        }
    }
}
