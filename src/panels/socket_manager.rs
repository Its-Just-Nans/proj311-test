use eframe::egui;
use ewebsock::{WsMessage, WsSender};
use std::cell::RefCell;
use std::rc::Rc;

pub struct SocketManager {
    ws_sender: Rc<RefCell<WsSender>>,
    text_to_send: String,
    msg_id: u32,
}

impl SocketManager {
    pub fn new(ws_sender: Rc<RefCell<WsSender>>) -> Self {
        Self {
            ws_sender: ws_sender,
            text_to_send: Default::default(),
            msg_id: 1,
        }
    }
}

impl super::PanelController for SocketManager {
    fn window_title(&self) -> &'static str {
        "Socket Manager"
    }
    fn name(&self) -> &'static str {
        "Socket Manager"
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

impl super::PanelView for SocketManager {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Previous").clicked() {
                log::info!("Previous");
            }
            if ui.button("Next").clicked() {
                log::info!("Next");
                // TODO message_id
                let msg = r#"
                            {"timeout":1,"min":64,"max":2048,"layers":{"PHY":"warn","MAC":"warn","RLC":"warn","PDCP":"warn","RRC":"debug","NAS":"debug","S72":"warn","S1AP":"warn","NGAP":"warn","GTPU":"warn","X2AP":"warn","XnAP":"warn","M2AP":"warn","LPPa":"warn","NRPPa":"warn","TRX":"warn"},"message":"log_get","headers":false,"message_id":1}
                            "#;
                self.ws_sender.borrow_mut().send(WsMessage::Text(msg.into()));
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Message to send:");
                if ui.text_edit_singleline(&mut self.text_to_send).lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    log::info!("Send message: {}", self.text_to_send);
                    self.ws_sender
                        .borrow_mut()
                        .send(WsMessage::Text(std::mem::take(&mut self.text_to_send)));
                }
            });

            ui.separator();
        });
    }
}
