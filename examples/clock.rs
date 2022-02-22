fn main() {
    let app = egui_app::FractalClock::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}