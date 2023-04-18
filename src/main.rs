mod complex;

use eframe::egui;
use egui::{TextureOptions, ColorImage, Slider, Vec2, Image, Sense};

use crate::complex::Complex;
#[derive(Default)]
struct MyApp {
    width: u32,
    height: u32,
    offset: Vec2,
    scale: f64,
    texture: Option<(egui::Vec2, egui::TextureHandle)>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load the image:
        let image = image::DynamicImage::new_rgb8(self.width, self.height);
        let mut image_buffer = image.to_rgba8();
        for (x, y, pixel) in image_buffer.enumerate_pixels_mut(){
            let x = x as f64 - (image.width() as f64) / 2.;
            let y = y as f64 - (image.height() as f64) / 2.;
            
            let c = Complex::from((x,y)) / self.scale + self.offset.into();
            let mut z = Complex::from((0.,0.));
            let mut val : u8 = 0;
            for i in 0..=50{
                z = z * z + c;
                if z.length_squared() > 4. || i == 50{ 
                    val = i*5;
                    break;
                }
                // if z.real.abs() < 1e-10 && z.imaginary.abs() < 1e-10{
                //     break;
                // }
            }
            pixel.0 = [val, val, val, 255 ];
        }
        let size = [image.width() as usize, image.height() as usize];
        let pixels = image_buffer.into_vec();
        assert_eq!(size[0] * size[1] * 4, pixels.len());
        let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
        // Allocate a texture:
        let texture = ctx.load_texture("Mandelbrot", image, TextureOptions::LINEAR);
        
        let size = egui::Vec2::new(size[0] as f32, size[1] as f32);
        self.texture = Some((size, texture));

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some((size, texture)) = self.texture.as_ref() {
                let response = ui.add(Image::new(texture.id(), *size).sense(Sense::drag()));
                self.offset -= response.drag_delta() / self.scale as f32;
            }
            // ui.add(
            //     Slider::new(&mut self.scale, 1. ..=10000.)
            //         .text("Scale")
            //         .clamp_to_range(true),
            // );
            ui.input(|i| self.scale *= 1.2_f64.powf(i.scroll_delta.y as f64 / 100.) )
        });
    }
}

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.fullscreen = true;
    eframe::run_native(
        "Mandelbrot", 
        options, 
        Box::new(|_| Box::new(MyApp{width: 800, height: 800, offset: Vec2::ZERO, scale: 1000., texture: None}))
    ).unwrap();
}