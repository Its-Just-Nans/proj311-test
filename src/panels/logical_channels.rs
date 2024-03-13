use eframe::egui;
use ewebsock::WsEvent;

#[derive(Default)]
pub struct LogicalChannels {
    ref_vec: Option<Vec<WsEvent>>,
}

impl LogicalChannels {
    pub fn new() -> Self {
        Self { ref_vec: None }
    }
    pub fn set_events(&mut self, events: Vec<WsEvent>) {
        self.ref_vec = Some(events);
    }
}

impl super::PanelController for LogicalChannels {
    fn name(&self) -> &'static str {
        "Canaux logiques"
    }
    fn window_title(&self) -> &'static str {
        "Téléphone - Canaux logiques (couche 3)"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.window_title())
            .default_width(320.0)
            .default_height(480.0)
            .open(open)
            .show(ctx, |ui| {
                use super::PanelView as _;
                self.ui(ui);
            });
    }
}

impl super::PanelView for LogicalChannels {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Received events:");
        ui.label("yolo");
    }
}
