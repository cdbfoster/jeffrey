//
// This file is part of Jeffrey.
//
// Jeffrey is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Jeffrey is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Jeffrey. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright 2017 Chris Foster
//

use std::slice::{Iter, IterMut};

use sdl2::EventPump;
use sdl2::event::Event;

pub struct EventQueue {
    event_pump: EventPump,
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new(event_pump: EventPump) -> EventQueue {
        EventQueue {
            event_pump: event_pump,
            events: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            self.events.push(event);
        }
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn remove(&mut self, index: usize) -> Event {
        self.events.remove(index)
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn iter(&self) -> Iter<Event> {
        self.events.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Event> {
        self.events.iter_mut()
    }
}
