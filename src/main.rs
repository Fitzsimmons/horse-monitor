use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use std::io;
use regex::Regex;

use lazy_static::lazy_static;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

lazy_static! {
    static ref SPEED_REGEX: Regex = Regex::new(r#"\{Base: ([\d\.]+)d, Name: "minecraft:generic\.movement_speed"\}"#).unwrap();
    // static ref SPEED_REGEX: Regex = Regex::new(r#"{Base: ([\d\.]+)d, Name: "minecraft:generic\.movement_speed"}"#).unwrap();
    static ref JUMP_REGEX: Regex = Regex::new(r#"\{Base: ([\d\.]+)d, Name: "minecraft:horse\.jump_strength"\}"#).unwrap();
    static ref HEALTH_REGEX: Regex = Regex::new(r#"\{Base: ([\d\.]+)d, Name: "minecraft:generic\.max_health"\}"#).unwrap();
}

const SPEED_MAX: f64 = 0.3375;
const HEALTH_MAX: f64 = 30.0;
const JUMP_MAX: f64 = 1.0;

struct Handler {
    ctx: ClipboardContext,
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let s = self.ctx.get_contents().unwrap();

        if let Some(speed_capture) = SPEED_REGEX.captures(&s) {
            let speed: f64 = speed_capture.get(1).unwrap().as_str().parse().unwrap();
            println!("Speed:  {0:.2}", speed / SPEED_MAX * 100.0);
        }

        if let Some(jump_capture) = JUMP_REGEX.captures(&s) {
            let jump: f64 = jump_capture.get(1).unwrap().as_str().parse().unwrap();
            println!("Jump:   {0:.2}", jump / JUMP_MAX * 100.0);
        }

        if let Some(health_capture) = HEALTH_REGEX.captures(&s) {
            let health: f64 = health_capture.get(1).unwrap().as_str().parse().unwrap();
            println!("Health: {0:.2}", health / HEALTH_MAX * 100.0);
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn main() {
    let h = Handler{ctx: ClipboardProvider::new().unwrap()};
    let _ = Master::new(h).run();
}
