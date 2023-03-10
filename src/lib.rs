mod clock;
mod cpu;
mod keypad;
mod font;

use std::fs;
use std::path::Path;

pub use cpu::{Pixel, PIXELS_WIDTH, PIXELS_HEIGHT};
pub use keypad::{Key, KeyState};
use clock::Clock;
use cpu::Cpu;

const DEFAULT_SPEED: u64 = 700;
const TIMER_FREQUENCY: f64 = 60.0;

pub struct Chip8 {
    cpu: Cpu,
    clock: Clock,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(DEFAULT_SPEED as f64 / TIMER_FREQUENCY),
            clock: Clock::new(DEFAULT_SPEED),
        }
    }

    pub fn start(&mut self) {
        self.clock.start();
    }

    pub fn stop(&mut self) {
        self.clock.stop();
    }

    pub fn tick(&mut self) {
        self.clock.tick();
    }

    pub fn set_speed(&mut self, instructions_per_second: u64) {
        self.cpu.set_timer_speed(instructions_per_second as f64 / TIMER_FREQUENCY);
        self.clock.set_speed(instructions_per_second);
    }

    pub fn pixels(&self) -> &[Pixel] {
        self.cpu.pixels()
    }

    pub fn load(&mut self, path: &Path) {
        let buffer = fs::read(path).unwrap();
        self.cpu.load_into_memory(0x200, buffer.as_slice());
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn handle_key_event(&mut self, key: Key, state: KeyState) {
        self.cpu.key_pad[key] = state;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.stop();
    }
}
