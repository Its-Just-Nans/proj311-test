use eframe::egui;
use ewebsock::{WsEvent, WsMessage};

#[derive(Default)]
pub struct MessageBox {
    ref_vec: Option<Vec<WsEvent>>,
}

impl MessageBox {
    pub fn new() -> Self {
        Self { ref_vec: None }
    }
    pub fn set_events(&mut self, events: Vec<WsEvent>) {
        self.ref_vec = Some(events);
    }
}

impl super::Demo for MessageBox {
    fn name(&self) -> &'static str {
        "message box "
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

impl super::View for MessageBox {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Received events:");
        if let Some(events) = &self.ref_vec {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for event in events.iter() {
                        let text = match event {
                            WsEvent::Message(WsMessage::Text(text)) => text,
                            _ => continue,
                        };
                        ui.label(text);
                    }
                });
        }
    }
}
