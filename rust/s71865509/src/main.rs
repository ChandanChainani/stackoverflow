use eframe::{epi, egui};

struct MyEguiApp {
    name: String,
    age: u32,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl epi::App for MyEguiApp {
   fn name(&self) -> &str {
       "Test"
   }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui aplication");
            ui.horizontal(|ui|{
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age,0..=120));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let app = MyEguiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
