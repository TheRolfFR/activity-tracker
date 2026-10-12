use iced::{padding, widget::row};

use crate::{Element, component::sensor_graph::SensorGraph, state::MainMessage};

pub struct ActivityGraphs {
    pub click_activity: SensorGraph,
    pub input_activity: SensorGraph,
}

impl ActivityGraphs {
    pub fn new() -> Self {
        Self {
            click_activity: SensorGraph::new("Click activity"),
            input_activity: SensorGraph::new("Input activity"),
        }
    }

    pub fn view(&self)-> Element<'_, MainMessage> {
        row![
            self.click_activity.view(),
            self.input_activity.view()
        ].spacing(11).padding(padding::horizontal(14).vertical(11)).into()
    }
}
