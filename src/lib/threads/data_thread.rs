use std::sync::mpsc::{Receiver, Sender};

use log::error;

use crate::{data::{ONE_MINUTE, ONE_SECOND, records::Record, payload::Payload}, storage::DayRecordFile, throttle::Throttle, week_data::WeekData};

pub enum DataMessage {
    EventRecord(Record),
    Adjustement(i32)
}

pub fn data_thread(evt_rchan: Receiver<DataMessage>, evt_schan: Sender<Payload>) -> ! {
    let mut week_data: WeekData = WeekData::load();

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
                    week_data.update_today(record);
                },
                DataMessage::Adjustement(adjusted) => {
                    week_data.adjust_today(adjusted);
                }
            }
        }

        if ui_throttle.update() {
            let payload = Payload::from(&week_data);
            if let Err(err) = evt_schan.send(payload) {
                error!("Failed to send payload from data thread: {}", err);
            }
        }

        if file_throttle.update() && let Some(today) = week_data.today() {
            DayRecordFile::save(today);
        }
    }
}
