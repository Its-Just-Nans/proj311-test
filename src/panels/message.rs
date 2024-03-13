use eframe::egui;
use ewebsock::{WsEvent, WsMessage};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct MessageBox {
    events: Rc<RefCell<Vec<WsEvent>>>,
}

impl MessageBox {
    pub fn new(ref_events: Rc<RefCell<Vec<WsEvent>>>) -> Self {
        Self { events: ref_events }
    }
}

impl super::PanelController for MessageBox {
    fn name(&self) -> &'static str {
        "Messages"
    }

    fn window_title(&self) -> &'static str {
        "Scoket Message"
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

impl super::PanelView for MessageBox {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Received events:");
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for event in self.events.borrow().iter() {
                    let text = match event {
                        WsEvent::Message(WsMessage::Text(text)) => text,
                        _ => continue,
                    };
                    ui.label(text);
                }
            });
    }
}
