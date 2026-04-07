use std::collections::HashMap;

use winit::{event::{ElementState, KeyEvent}, keyboard::{KeyCode, PhysicalKey}};

struct KeyInfo {
    pressed_this_frame: bool,
    released_this_frame: bool,
    held: bool
}

impl KeyInfo {
    pub fn new() -> Self {
        Self { pressed_this_frame: false, released_this_frame: false, held: false }
    }
}

pub struct Input {
    key_map: HashMap<KeyCode, KeyInfo>
}

impl Input {
    pub fn new() -> Self {
        Input {
            key_map: HashMap::new()
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        if event.repeat {
            return;
        }

        match event.physical_key {
            PhysicalKey::Code(code) => {
                let info = self.key_map.entry(code).or_insert_with(|| KeyInfo::new());
            
                match event.state {
                    ElementState::Pressed => {
                        info.pressed_this_frame = true;
                        info.held = true;
                    },
                    ElementState::Released => {
                        info.released_this_frame = true;
                        info.held = false;
                    }
                }
            },
            _ => {}
        }        
    }

    pub fn end_update(&mut self) {
        for (_, info) in &mut self.key_map {
            info.pressed_this_frame = false;
            info.released_this_frame = false;
        }
    }

    pub fn get_key(&self, key: KeyCode) -> bool {
        match self.key_map.get(&key) {
            Some(info) => info.held,
            None => false
        }
    }

    pub fn get_key_down(&self, key: KeyCode) -> bool {
        match self.key_map.get(&key) {
            Some(info) => info.pressed_this_frame,
            None => false
        }
    }

    pub fn get_key_up(&self, key: KeyCode) -> bool {
        match self.key_map.get(&key) {
            Some(info) => info.released_this_frame,
            None => false
        }
    }
}