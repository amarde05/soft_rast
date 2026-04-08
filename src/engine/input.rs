use std::collections::HashMap;

use glam::Vec2;
use winit::{dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, KeyEvent, MouseButton}, keyboard::{KeyCode, PhysicalKey}};

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
    key_map: HashMap<KeyCode, KeyInfo>,

    mouse_map: HashMap<MouseButton, KeyInfo>,
    mouse_pos: Vec2,
    mouse_delta: Vec2,
}

impl Input {
    pub fn new() -> Self {
        Input {
            key_map: HashMap::new(),

            mouse_map: HashMap::new(),
            mouse_pos: Vec2 { x: 0., y: 0. },
            mouse_delta: Vec2 { x: 0., y: 0. }
        }
    }

    pub fn end_update(&mut self) {
        for (_, info) in &mut self.key_map {
            info.pressed_this_frame = false;
            info.released_this_frame = false;
        }

        for (_, info) in &mut self.mouse_map {
            info.pressed_this_frame = false;
            info.released_this_frame = false;
        }

        self.mouse_delta = Vec2::ZERO;
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


    pub fn handle_mouse_event(&mut self, state: ElementState, button: MouseButton) {
        let info = self.mouse_map.entry(button).or_insert_with(|| KeyInfo::new());

        match state {
            ElementState::Pressed => {
                info.pressed_this_frame = true;
                info.held = true;
            },
            ElementState::Released => {
                info.released_this_frame = true;
                info.held = false;
            }
        }
    }

    pub fn handle_cursor_movement(&mut self, position: PhysicalPosition<f64>, window_size: PhysicalSize<u32>) {
        self.mouse_delta = Vec2::new(
            (position.x as f32 - self.mouse_pos.x), /// window_size.width as f32,
            (position.y as f32 - self.mouse_pos.y)// / window_size.height as f32
        );
        self.mouse_pos = Vec2::new(position.x as f32, position.y as f32);
    }

    pub fn handle_cursor_delta(&mut self, delta: (f64, f64)) {
        self.mouse_delta = Vec2::new(delta.0 as f32, delta.1 as f32);
    }

    pub fn get_mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }

    pub fn get_mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn get_mouse_button(&self, button: MouseButton) -> bool {
        match self.mouse_map.get(&button) {
            Some(info) => info.held,
            None => false
        }
    }

    pub fn get_mouse_button_down(&self, button: MouseButton) -> bool {
        match self.mouse_map.get(&button) {
            Some(info) => info.pressed_this_frame,
            None => false
        }
    }

    pub fn get_mouse_up(&self, button: MouseButton) -> bool {
        match self.mouse_map.get(&button) {
            Some(info) => info.released_this_frame,
            None => false
        }
    }
}