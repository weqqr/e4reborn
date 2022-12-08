use std::fmt::Formatter;
use eframe::egui;

pub struct Instruction {
    pub data: u16,
    pub opcode: u16,
    pub payload: u16,
    pub hint: Option<String>,
}

impl Instruction {
    pub fn decode(data: u16, hint: Option<String>) -> Self {
        let opcode = data >> 12;
        let payload = data & 0b111111111111;
        Self { data, opcode, payload, hint }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(hint) = &self.hint {
            return write!(f, "{}", hint);
        }

        match self.opcode {
            0b0000 => write!(f, "load"),
            0b0001 => write!(f, "store"),
            0b0010 => write!(f, "loadc"),
            0b0011 => write!(f, "storec"),
            0b0100 => write!(f, "add"),
            0b0101 => write!(f, "addc"),
            0b0110 => write!(f, "jz"),
            0b0111 => write!(f, "jmp"),
            0b1000 => write!(f, "halt"),
            _ => write!(f, "unrecognized opcode"),
        }
    }
}

pub struct Memory {
    data: Vec<u16>,
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("E4 Reborn", native_options, Box::new(|cc| Box::new(App::new(cc))));
}

#[derive(Default)]
struct App {}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}