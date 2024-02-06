// src/coeditor/mod.rs

use eframe::egui;
use pyo3::prelude::*;
//use pyo3::types::IntoPyDict;
use egui::vec2;

#[derive(Default)]
pub struct MyEguiApp {
    open_editor: bool,
    code: String,
}

impl eframe::App for MyEguiApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

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
                let fixed_size = vec2(350.0, 350.0);
               // let ainputt: bool = ctx.input(|i: &egui::InputState| i.key_pressed(egui::Key::A));
                if ctx.input(|i| i.key_pressed(egui::Key::Tab)) {
                    self.code.push('\t');
                    }
                //if ainputt && kinputt {
                //    println!("hi")
                //}
                egui::Window::new("editor")
                
                .open(&mut self.open_editor)
                .fixed_size(fixed_size)
                .show(ctx, |ui| {
                   
                    ui.text_edit_multiline(&mut self.code);
                    
                    if ui.button("run").clicked() {
                        
                        pyo3::prepare_freethreaded_python();
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
