use std::{thread, sync::mpsc::{Receiver, Sender}};
use bounded_vec_deque::BoundedVecDeque;
use rdev::{EventType::*, Event};
use std::time::Instant;

use crate::data::*;

pub fn activity_thread(evt_rchan: Receiver<Event>, act_schan: Sender<Activity>) {
    // event recorder
    thread::spawn(move || -> ! {
        let mut clicks_by_minutes: BoundedVecDeque::<usize> = BoundedVecDeque::from_iter([0], 5);
        let mut last_click = Instant::now();

        let mut inputs_per_min: BoundedVecDeque::<usize> = BoundedVecDeque::from_iter([0], 300);
        let mut last_pres = last_click.clone();

        let mut last_emit = last_click.clone();

        loop {
            let recv_result = evt_rchan.recv_timeout(ONE_SECOND);

            let can_emit = if let Ok(event) = recv_result {
                let now = Instant::now();
                if let KeyPress(_k) = event.event_type {
                    let mut diff = now.duration_since(last_pres);
                    if diff >= ONE_MINUTE {
                        // fill empty seconds
                        while diff >= ONE_MINUTE {
                            inputs_per_min.push_back(0);
                            diff -= ONE_MINUTE;
                        }
                        last_pres = now;
                    }

                    let last = inputs_per_min.back_mut().unwrap(); // one element at least
                    *last += 1;
                }

                if let ButtonPress(_b) = event.event_type {
                    let mut diff = now.duration_since(last_click);
                    if now.duration_since(last_click) >= ONE_MINUTE {
                        // fill empty minutes
                        while diff >= ONE_MINUTE {
                            clicks_by_minutes.push_back(0);
                            diff -= ONE_MINUTE;
                        }
                        last_click = now;
                    }

                    let last = clicks_by_minutes.back_mut().unwrap(); // one element at least
                    *last += 1;
                }

                if now.duration_since(last_emit) >= ONE_SECOND {
                    last_emit = now;
                    true
                } else {
                    false
                }
            } else {
                true
            };
            
            if can_emit {
                let mut clicks = clicks_by_minutes.iter().fold((0usize,0usize), |mut acc, cur| {
                    acc.0 += cur;
                    acc.1 += 1;

                    acc
                });
                clicks.1 = clicks.1.max(1);

                let mut inputs = inputs_per_min.iter().fold((0usize,0usize), |mut acc, cur| {
                    if cur > &10 {
                        acc.0 += cur;
                        acc.1 += 1;
                    }
                    
                    acc
                });
                inputs.1 = inputs.1.max(1);

                let act = Activity {
                    clicks_per_minute: (clicks.0 as f32)/(clicks.1 as f32),
                    click_series: ActivitySeries {
                        points: Vec::from_iter(clicks_by_minutes.clone()),
                        labels: CLICK_LABELS
                    },
                    inputs_per_minute: (inputs.0 as f32)/(inputs.1 as f32),
                    input_series: ActivitySeries {
                        points: Vec::from_iter(inputs_per_min.clone()),
                        labels: KEY_LABELS
                    },
                };

                act_schan.send(act).ok();
            }
        }
    });
}