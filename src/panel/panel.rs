use eframe::egui;

#[derive(Default)]
pub struct ConnectionPanel {}

impl super::Demo for ConnectionPanel {
    fn name(&self) -> &'static str {
        "About egui"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            .open(open)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for ConnectionPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};

        ui.heading("egui");
        ui.label(format!(
            "egui is an immediate mode GUI library written in Rust. egui runs both on the web and natively on {}{}{}. \
            On the web it is compiled to WebAssembly and rendered with WebGL.{}",
            OS_APPLE, OS_LINUX, OS_WINDOWS,
            if cfg!(target_arch = "wasm32") {
                " Everything you see is rendered as textured triangles. There is no DOM, HTML, JS or CSS. Just Rust."
            } else {""}
        ));

        ui.add_space(12.0); // ui.separator();
        ui.heading("Links");

        ui.add_space(12.0);

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("egui development is sponsored by ");
            ui.hyperlink_to("Rerun.io", "https://www.rerun.io/");
            ui.label(", a startup building an SDK for visualizing streams of multimodal data.");
        });

        ui.add_space(12.0);
    }
}
