use std::sync::mpsc::Receiver;

use crate::data::{Activity, FIVE_MINUTES};
use crate::day_activity::DayActivity;

pub fn thread_activity_week(evt_rchan: Receiver<Activity>) {
    loop {
        let recv_result = evt_rchan.recv_timeout(FIVE_MINUTES);
        
        if let Ok(act) = recv_result {
            DayActivity::update(act);
        } else {
            DayActivity::stop();
        }
    }
}