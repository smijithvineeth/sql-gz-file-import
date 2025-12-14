use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "My First Rust GUI",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    name: String,
    count: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello Rust GUI 👋");

            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.name);
            });

            if ui.button("Click me").clicked() {
                self.count += 1;
            }

            ui.label(format!("Clicked {} times", self.count));

            if !self.name.is_empty() {
                ui.label(format!("Hello, {}", self.name));
            }
        });
    }
}
