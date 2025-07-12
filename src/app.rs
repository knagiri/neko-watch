use chrono::{DateTime, Local};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

use crate::cat::Cat;

pub struct App {
    pub cat: Cat,
    pub should_quit: bool,
    last_update: DateTime<Local>,
    #[cfg(debug_assertions)]
    pub show_debug: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            cat: Cat::new("ネコ".to_string()),
            should_quit: false,
            last_update: Local::now(),
            #[cfg(debug_assertions)]
            show_debug: false,
        }
    }

    pub fn tick(&mut self) {
        let now = Local::now();
        let delta = now.signed_duration_since(self.last_update);
        let delta_seconds = delta.num_milliseconds() as f64 / 1000.0;
        
        self.cat.update(delta_seconds);
        self.last_update = now;
    }

    pub fn handle_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Char('1') => {
                        if self.cat.can_perform_action() {
                            self.cat.feed();
                        }
                    }
                    KeyCode::Char('2') => {
                        if self.cat.can_perform_action() {
                            self.cat.play();
                        }
                    }
                    KeyCode::Char('3') => {
                        if self.cat.can_perform_action() {
                            self.cat.bathe();
                        }
                    }
                    KeyCode::Char('4') => {
                        if self.cat.can_perform_action() {
                            self.cat.sleep();
                        }
                    }
                    // デバッグキー (デバッグビルドのみ)
                    #[cfg(debug_assertions)]
                    KeyCode::Char('d') => {
                        self.show_debug = true;
                    }
                    #[cfg(debug_assertions)]
                    KeyCode::Char('0') => {
                        self.cat.set_status_for_test(5, 5, 5, 5); // 瀕死状態
                    }
                    #[cfg(debug_assertions)]
                    KeyCode::Char('8') => {
                        self.cat.set_status_for_test(15, 50, 50, 15); // 病気状態
                    }
                    #[cfg(debug_assertions)]
                    KeyCode::Char('9') => {
                        self.cat.set_status_for_test(15, 50, 50, 50); // 空腹状態
                    }
                    #[cfg(debug_assertions)]
                    KeyCode::Char('h') => {
                        self.show_debug = false; // デバッグ情報を非表示
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}