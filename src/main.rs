use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_plot NaN Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp))),
    )
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let values = [1.0, 2.0, 3.0, f64::NAN, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

            let points: PlotPoints = values
                .iter()
                .enumerate()
                .map(|(i, &y)| [i as f64, y])
                .collect();

            Plot::new("line_plot").show(ui, |plot_ui| {
                plot_ui.line(Line::new("series", points));
            });
        });
    }
}
