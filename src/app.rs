use eframe::egui::{self};
use ewebsock::{WsEvent, WsReceiver, WsSender};
use std::rc::Rc;
use std::{cell::RefCell, collections::BTreeSet};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use crate::panels::{
    AboutPanel, FileHandler, LogicalChannels, MessageBox, PanelController, SocketManager,
};

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
            url: "ws://137.194.194.51:9001".to_owned(),
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
                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
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
            egui::CentralPanel::default().show(ctx, |ui| ui.horizontal(|ui| ui.vertical(|_ui| {})));
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
    ws_sender: Rc<RefCell<WsSender>>,
    ws_receiver: Rc<RefCell<WsReceiver>>,
    events: Rc<RefCell<Vec<WsEvent>>>,
    pub open_windows: BTreeSet<String>,
    pub windows: Vec<Box<dyn PanelController>>,
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
        let ref_ws_sender = Rc::new(RefCell::new(ws_sender));
        let ref_ws_receiver = Rc::new(RefCell::new(ws_receiver));
        let ref_events = Rc::new(RefCell::new(Vec::new()));
        let mb = MessageBox::new(Rc::clone(&ref_events));
        let sm = SocketManager::new(Rc::clone(&ref_ws_sender));
        Self {
            ws_sender: ref_ws_sender,
            ws_receiver: ref_ws_receiver,
            events: ref_events,
            open_windows: BTreeSet::new(),
            windows: vec![
                Box::<AboutPanel>::default(),
                Box::<MessageBox>::new(mb),
                Box::<FileHandler>::default(),
                Box::<LogicalChannels>::default(),
                Box::<SocketManager>::new(sm),
            ],
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        while let Some(event) = self.ws_receiver.borrow_mut().try_recv() {
            self.events.borrow_mut().push(event);
        }
        for one_window in self.windows.iter_mut() {
            let mut is_open: bool = self.open_windows.contains(one_window.name());
            one_window.show(ctx, &mut is_open);
            set_open(&mut self.open_windows, one_window.name(), is_open);
        }
        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
