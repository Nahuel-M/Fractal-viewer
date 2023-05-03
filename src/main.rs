#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod fractal;
mod color;

use std::sync::{Mutex, Arc};

use color::complementary_color;
use eframe::{egui, egui_glow};
use egui::{Response, Vec2, Slider, Grid, Frame, epaint::Shadow, Visuals, Color32};
use fractal::Fractal;

use crate::color::rotate_hue;


struct FractalVisualizer {
    state: State,
    fractal: Arc<Mutex<Fractal>>,
    disco_mode: bool,
    fps: f32,
}
#[derive(Default, Clone, Copy)]
pub struct State{
    pub offset: Vec2,
    pub scale: f32,
    pub iterations: u32,
    pub start_color: [f32; 3],
    pub end_color: [f32; 3],
}

impl FractalVisualizer{
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc.gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            fractal: Arc::new(Mutex::new(Fractal::new(gl))),
            disco_mode: false,
            fps: 60.,
            state: State{
                offset: Vec2::ZERO,
                scale: 0.001,
                iterations: 250,
                start_color: [0.05, 0.05, 0.2],
                end_color: [1., 1., 0.],
            }
        }
    }

    fn custom_painting(&mut self, ui: &mut egui::Ui, size: Vec2, pixels_per_point: f32) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(size, egui::Sense::drag());

       
        // Clone locals so we can move them
        let fractal = self.fractal.clone();
        let state = self.state;
        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(
                move |_info, painter| {
                fractal.lock().unwrap().paint(painter.gl(), &state, size*pixels_per_point);
            })),
        };
        ui.painter().add(callback);

        response
    }
}


impl eframe::App for FractalVisualizer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        ctx.set_visuals(Visuals{
            window_shadow: Shadow{
                extrusion: 10.0,
                color: Color32::from_black_alpha(30),
                },
            ..Default::default()
        });

        egui::CentralPanel::default()
            .frame(Frame::default().inner_margin(0.).outer_margin(0.))
            .show(ctx, |ui| {

            egui::Frame::canvas(ui.style()).inner_margin(0.).outer_margin(0.).show(ui, |ui| {
                let response = self.custom_painting(ui, ctx.screen_rect().size(), frame.info().native_pixels_per_point.unwrap());
                self.state.offset += Vec2::new(1., -1.) * 2. * response.drag_delta() * self.state.scale;

                // Scaling
                ui.input(|i| {
                    let scaling = i.zoom_delta() * 1.2_f32.powf(i.scroll_delta.y / 100.);
                    if let Some(mouse_position) = i.pointer.hover_pos(){
                        self.state.scale /= scaling;
                        let mut mouse_position = mouse_position.to_vec2() / ctx.screen_rect().size() - Vec2::splat(0.5);
                        mouse_position.y *= -1.; // Flip y to coordinate frame;
                        self.state.offset += mouse_position * (1. - scaling) * self.state.scale * ctx.screen_rect().size() * 2.;
                    }
                });
            });

            egui::Window::new("Settings").resizable(false).show(ctx, |ui|{
                Grid::new("settings").striped(true).show(ui, |ui| {
                    ui.label("Iterations: ");
                    ui.add(Slider::new(&mut self.state.iterations, 1..=500).clamp_to_range(false));
                    ui.end_row();

                    ui.label("Primary colors: ");
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut self.state.start_color);
                        ui.color_edit_button_rgb(&mut self.state.end_color);
                    });
                    ui.end_row();

                    ui.label("Disco mode??");
                    if ui.checkbox(&mut self.disco_mode, "").clicked() && self.disco_mode{
                        self.state.start_color = [1.0, 0., 0.,];
                    };
                    ui.end_row();

                    ui.input(|i| self.fps = 0.9 * self.fps + 0.1*(1./i.stable_dt));
                    ui.label("Fps: ");
                    ui.label(self.fps.round().to_string());
                    ui.end_row();
                    ui.hyperlink_to("Github", "https://github.com/Nahuel-M/Fractal-viewer");
                });
            });

            // Disco mode
            if self.disco_mode{
                rotate_hue(&mut self.state.start_color, 0.05);
                self.state.end_color = complementary_color(self.state.start_color);
                ctx.request_repaint();
            }
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.fractal.lock().unwrap().destroy(gl);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    
     wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas", 
            web_options, 
            Box::new(|cc| Box::new(FractalVisualizer::new(cc))),
        )
        .await
        .unwrap();
        
    });
}

// #[cfg(not(target_arch = "wasm32"))]
// fn main() -> eframe::Result<()> {
//     // Log to stdout (if you run with `RUST_LOG=debug`).
//     tracing_subscriber::fmt::init();

//     let options = eframe::NativeOptions{
//         fullscreen: true,
//         multisampling: 4,
//         renderer: eframe::Renderer::Glow,
//         ..Default::default()
//     };

//     eframe::run_native(
//         "Mandelbrot",
//         options,
//         Box::new(|cc| Box::new(FractalVisualizer::new(cc, 900, 900))),
//     )
// }