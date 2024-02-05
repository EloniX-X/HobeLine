// src/coeditor/mod.rs

use eframe::egui;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;


#[derive(Default)]
pub struct MyEguiApp {
    open_editor: bool,
    code: String,
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hello this is the coeditor");
            ui.horizontal(|ui| {
                ui.label("how are you");
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                println!("closing");
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            };
            if ui.button("open coeditor").clicked() {
                self.open_editor = true;
            }
        });
    
            if self.open_editor {
                egui::Window::new("editor")
                .open(&mut self.open_editor)
                .show(ctx, |ui| {
                    ui.text_edit_multiline(&mut self.code);
                    if ui.button("run").clicked() {
                        println!("runnn");
                        println!("{}", self.code);
                        Python::with_gil(|py| {
                            if let Err(e) = py.run(&self.code, None, None) {
                                eprintln!("Error executing Python code: {}", e);
                                return; 
                            }
                        });
                
                    };

                });
            }

        }
    }
