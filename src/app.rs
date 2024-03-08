use eframe::egui::{self};
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use std::collections::BTreeSet;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use crate::panel::{ConnectionPanel, Demo, FileHandler, MessageBox};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ExampleApp {
    pub url: String,
    #[serde(skip)]
    pub error: String,
    #[serde(skip)]
    frontend: Option<FrontEnd>,
}

impl ExampleApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            url: "ws://137.194.194.51:9000".to_owned(),
            error: Default::default(),
            frontend: None,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.menu_button("File", |ui| {
                    if ui.button("Upload a file").clicked() {
                        // TODO open file dialog
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                if let Some(current_frontend) = &mut self.frontend {
                    ui.menu_button("Windows", |ui| {
                        for one_window in current_frontend.windows.iter_mut() {
                            let mut is_open: bool =
                                current_frontend.open_windows.contains(one_window.name());
                            ui.checkbox(&mut is_open, one_window.name());
                            set_open(
                                &mut current_frontend.open_windows,
                                one_window.name(),
                                is_open,
                            );
                        }
                        if ui.button("Organize windows").clicked() {
                            ui.ctx().memory_mut(|mem| mem.reset_areas());
                        }
                    });
                }
            });
        });

        egui::TopBottomPanel::top("server").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("URL:");
                if ui.text_edit_singleline(&mut self.url).lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    self.connect(ctx.clone());
                }
                if self.frontend.is_some() {
                    // its is connected
                    if ui.button("Close").clicked() {
                        // TODO close connection
                        self.frontend = None;
                    }
                }
            });
        });

        if !self.error.is_empty() {
            egui::TopBottomPanel::top("error").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Error:");
                    ui.colored_label(egui::Color32::RED, &self.error);
                });
            });
        }

        if let Some(frontend) = &mut self.frontend {
            frontend.ui(ctx);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Connect to WebScoket to start");
                    })
                })
            });
        }
    }
}

impl ExampleApp {
    fn connect(&mut self, ctx: egui::Context) {
        let wakeup = move || ctx.request_repaint(); // wake up UI thread on new message
        let options = ewebsock::Options {
            max_incoming_frame_size: 500,
        };
        match ewebsock::connect_with_wakeup(&self.url, options, wakeup) {
            Ok((ws_sender, ws_receiver)) => {
                self.frontend = Some(FrontEnd::new(ws_sender, ws_receiver));
                self.error.clear();
            }
            Err(error) => {
                log::error!("Failed to connect to {:?}: {}", &self.url, error);
                self.error = error;
            }
        }
    }
}

struct FrontEnd {
    ws_sender: WsSender,
    ws_receiver: WsReceiver,
    events: Vec<WsEvent>,
    text_to_send: String,
    pub open_windows: BTreeSet<String>,
    pub windows: Vec<Box<dyn Demo>>,
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

impl FrontEnd {
    fn new(ws_sender: WsSender, ws_receiver: WsReceiver) -> Self {
        Self {
            ws_sender,
            ws_receiver,
            events: Default::default(),
            text_to_send: Default::default(),
            open_windows: BTreeSet::new(),
            windows: vec![
                Box::<ConnectionPanel>::default(),
                Box::<MessageBox>::default(),
                Box::<FileHandler>::default(),
            ],
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        while let Some(event) = self.ws_receiver.try_recv() {
            self.events.push(event);
        }
        for one_window in self.windows.iter_mut() {
            let mut is_open: bool = self.open_windows.contains(one_window.name());
            one_window.show(ctx, &mut is_open);
            set_open(&mut self.open_windows, one_window.name(), is_open);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Message to send:");
                if ui.text_edit_singleline(&mut self.text_to_send).lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    self.ws_sender
                        .send(WsMessage::Text(std::mem::take(&mut self.text_to_send)));
                }
            });

            ui.separator();
        });
    }
}
