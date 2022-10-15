use std::sync::mpsc::{Receiver, Sender};
use bounded_vec_deque::BoundedVecDeque;
use rdev::{EventType::*, Event};
use std::time::Instant;

use crate::{data::*, day_activity::DayActivity};

pub fn activity_thread(evt_rchan: Receiver<Event>, act_schan: Sender<Activity>, day_schan: Sender<Activity>) {
    // event recorder

    let click_labels: ActivityLabels = ActivityLabels {
        x: String::from("Clicks"),
        y: String::from("min")
    };
    let key_labels: ActivityLabels = ActivityLabels {
        x: String::from("Inputs"),
        y: String::from("sec")
    };

    let (click, input) = DayActivity::load_activity_today().get_last_activity();
    
    let mut clicks_by_minutes: BoundedVecDeque::<Measure::<usize>> = BoundedVecDeque::from_iter(click, MINUTES_PER_DAY);
    let mut last_click = Instant::now();

    let mut inputs_per_min: BoundedVecDeque::<Measure::<usize>> = BoundedVecDeque::from_iter(input, MINUTES_PER_DAY);
    let mut last_pres = last_click.clone();

    let mut last_emit = last_click.clone();

    let mut last_update = last_click.clone();

    loop {
        let recv_result = evt_rchan.recv_timeout(ONE_SECOND);

        let (can_emit, can_update_day) = if let Ok(event) = recv_result {
            let now = Instant::now();
            if let KeyPress(_k) = event.event_type {
                let mut diff = now.duration_since(last_pres);
                if diff >= ONE_MINUTE {
                    // fill empty seconds
                    while diff >= ONE_MINUTE {
                        inputs_per_min.push_back(Measure::new());
                        diff -= ONE_MINUTE;
                    }
                    last_pres = now;
                }

                if inputs_per_min.is_empty() {
                    inputs_per_min.push_back(Measure::new());
                }
                // impossible to make back_mut or_then due to double mutable borrow
                let last = inputs_per_min.back_mut().unwrap();
                
                (*last).count += 1;
            }

            if let ButtonPress(_b) = event.event_type {
                let mut diff = now.duration_since(last_click);
                if now.duration_since(last_click) >= ONE_MINUTE {
                    // fill empty minutes
                    while diff >= ONE_MINUTE {
                        clicks_by_minutes.push_back(Measure::new());
                        diff -= ONE_MINUTE;
                    }
                    last_click = now;
                }

                if clicks_by_minutes.is_empty() {
                    clicks_by_minutes.push_back(Measure::new());
                }

                let last = clicks_by_minutes.back_mut().unwrap(); // one element at least
                (*last).count += 1;
            }

            (if now.duration_since(last_emit) >= ONE_SECOND {
                last_emit = now;
                true
            } else {
                false
            }, if now.duration_since(last_update) >= ONE_MINUTE {
                last_update = now;
                true
            } else {
                false
            })
        } else {
            (true, false)
        };

        if can_emit || can_update_day {
            let mut clicks = clicks_by_minutes.iter().fold((0usize,0usize), |mut acc, cur| {
                acc.0 += cur.count;
                acc.1 += 1;

                acc
            });
            clicks.1 = clicks.1.max(1);

            let mut inputs = inputs_per_min.iter().fold((0usize,0usize), |mut acc, cur| {
                if cur.count > 10 {
                    acc.0 += cur.count;
                    acc.1 += 1;
                }
                
                acc
            });
            inputs.1 = inputs.1.max(1);

            let act = Activity {
                clicks_per_minute: (clicks.0 as f32)/(clicks.1 as f32),
                click_series: ActivitySeries {
                    points: Vec::from_iter(clicks_by_minutes.clone()),
                    labels: click_labels.clone()
                },
                inputs_per_minute: (inputs.0 as f32)/(inputs.1 as f32),
                input_series: ActivitySeries {
                    points: Vec::from_iter(inputs_per_min.clone()),
                    labels: key_labels.clone()
                },
            };
        
            if can_emit {
                act_schan.send(act.clone()).ok();
            }
    
            if can_update_day {
                day_schan.send(act).ok();
            }
        }
    }
}