use crate::gamestate::GameState;

use std::io::stdout;
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};

pub struct GameLoop {
    game_state: GameState,
    frame_rate: i32,
    running: bool,
}

impl GameLoop {
    pub fn new() -> Self {
        Self {
            game_state: GameState::gosper_glider_gun(),
            frame_rate: 6,
            running: true,
        }
    }

    #[allow(dead_code)]
    pub fn init_state_from_csv(&mut self, init_state_csv: &str) {
        self.game_state = GameState::load_from_csv(init_state_csv);
    }

    fn setup(&self) {
        execute!(stdout(), EnterAlternateScreen, Hide).unwrap();
        terminal::enable_raw_mode().unwrap();
    }

    fn cleanup(&self) {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
    }

    fn frame_rate_up(&mut self) {
        let step = 6;
        let max_fps = 144;
        if self.frame_rate < step {
            self.frame_rate = step;
        } else if self.frame_rate + step > max_fps {
            self.frame_rate = max_fps
        } else {
            self.frame_rate += step
        }
        self.refresh_frame();
    }

    fn frame_rate_down(&mut self) {
        let step = 6;
        let min_fps = 1;
        if (self.frame_rate - step) < min_fps {
            self.frame_rate = min_fps;
        } else {
            self.frame_rate -= step;
        }
        self.refresh_frame();
    }

    fn toggle_grid(&mut self) {
        self.game_state.toggle_grid();
        self.refresh_frame();
    }

    fn previous_frame(&mut self) {
        self.game_state.revert();
        self.refresh_frame();
    }

    fn refresh_frame(&mut self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        self.game_state.print();
        self.print_help();
    }

    fn next_frame(&mut self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        self.game_state.update_and_print();
        self.print_help();
    }

    fn print_help(&mut self) {
        let help = "Quit: q/Esc/^C | Pause/Resume: Space | Toggle grid: g\r\n \
                    Previous step: p | Next step: n | Slow down: - | Speed up: +\r\n";
        if self.running {
            print!(" \x1b[1;95mFrame rate: {} fps\x1b[0m\r\n", self.frame_rate);
        } else {
            print!(" \x1b[9;95mFrame rate: {} fps\x1b[0m\r\n", self.frame_rate);
        }
        print!(" \x1b[1;94m{}\x1b[0m", help);
    }

    fn start_game_loop(&mut self) {
        loop {
            if event::poll(Duration::from_millis(1000 / (self.frame_rate as u64))).unwrap() {
                match event::read().unwrap() {
                    Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) => match (code, modifiers) {
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            break; // Quit
                        }
                        (KeyCode::Char('q') | KeyCode::Esc, _) => {
                            break; // Quit
                        }
                        (KeyCode::Char('p'), _) => {
                            self.running = false;
                            self.previous_frame();
                        }
                        (KeyCode::Char('n'), _) => {
                            self.running = false;
                            self.next_frame();
                        }
                        (KeyCode::Char(' '), _) => {
                            self.running = !self.running;
                            self.refresh_frame();
                        }
                        (KeyCode::Char('g'), _) => {
                            self.toggle_grid();
                        }
                        (KeyCode::Char('+'), _) => {
                            self.frame_rate_up();
                        }
                        (KeyCode::Char('-') | KeyCode::Char('_'), _) => {
                            self.frame_rate_down();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            if self.running {
                self.next_frame();
            }
        }
    }

    pub fn run(&mut self) {
        self.setup();
        self.start_game_loop();
        self.cleanup();
    }
}
