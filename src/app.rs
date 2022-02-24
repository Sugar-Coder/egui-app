use eframe::{egui, epi};
use eframe::egui::Pos2;

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
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        let Self { rest_time, working_time, timer, fractal_clock} = self;

        timer.processing();

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 frame.quit();
        //             }
        //         });
        //     });
        // });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");
        //
        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(&mut "hello".to_string());
        //     });
        //
        //     ui.add(egui::Slider::new(rest_time, 0..=10).text("rest_time"));
        //     if ui.button("Increment").clicked() {
        //         *rest_time += 1;
        //         *working_time += 1;
        //     }
        //
        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.add(egui::Slider::new(rest_time, 0..=10).text("rest_time"));
            // ctx.request_repaint(); // called in fractal_clock.ui()

            fractal_clock.ui(ui, crate::seconds_since_midnight());

            egui::warn_if_debug_build(ui);
        });

        egui::Window::new(format!("{} {:02}:{:02}", timer.text, timer.time / 60, timer.time % 60)).default_pos(Pos2::new(100.0, 10.0)).show(ctx, |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.add(egui::Slider::new(working_time, 25..=45).text("min Working Time"));
            ui.add(egui::Slider::new(rest_time, 5..=15).text("min Rest Time"));
            if ui.button("Next").clicked() {
                timer.next(*working_time * 60, *rest_time * 60);
            }
        });
    }
}
