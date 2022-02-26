use eframe::{egui, epi};
use eframe::egui::{Id, Pos2};
use chrono::Timelike;

mod app_timer;
use app_timer::Timer;
use crate::FractalClock;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // minutes number
    rest_time: u32,
    working_time: u32,

    fractal_clock: FractalClock,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    timer: Timer,
}


impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            rest_time: 5,
            working_time: 25,
            timer: Timer::new(),
            fractal_clock: FractalClock::default(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "🦀️Rumato🍅"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        self.timer.setup(self.working_time * 60, self.rest_time * 60);
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { rest_time, working_time, timer, fractal_clock} = self;

        timer.processing();

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.add(egui::Slider::new(rest_time, 0..=10).text("rest_time"));
            // ctx.request_repaint(); // called in fractal_clock.ui()
            let mut finish_time = None;
            if timer.status == app_timer::Status::Working || timer.status == app_timer::Status::Relaxing {
                let finish_nativetime = timer.finished_at.time();
                finish_time = Some(finish_nativetime.num_seconds_from_midnight() as f64 + 1e-9 * (finish_nativetime.nanosecond() as f64));
            }
            fractal_clock.ui(ui, crate::seconds_since_midnight(), finish_time);

            egui::warn_if_debug_build(ui);
        });
        // The window title is used as a unique Id and must be unique, and should not change.
        // This is true even if you disable the title bar with .title_bar(false).
        // If you need a changing title, you must call window.id(…) with a fixed id.
        egui::Window::new(format!("{} {:02}:{:02}", timer.text, timer.time / 60, timer.time % 60))
            .id(Id::new("TimerWindow"))
            .default_pos(Pos2::new(100.0, 10.0))
            .resizable(false)
            .show(ctx, |ui| {
                // ctx.memory_ui(ui);
                // ui.label(format!("{} {:02}:{:02}", timer.text, timer.time / 60, timer.time % 60));
                ui.add(egui::Slider::new(working_time, 25..=45).text("min Working Time"));
                ui.add(egui::Slider::new(rest_time, 5..=15).text("min Rest Time"));
                ui.horizontal(|ui| {
                    if ui.button("Next").clicked() {
                        timer.next(*working_time * 60, *rest_time * 60);
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
    }
}
