use eframe::egui;
use poll_promise::Promise;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct FileHandler {
    pub dropped_files: Vec<egui::DroppedFile>,
    #[serde(skip)]
    pub picked_path: Option<String>,
    #[serde(skip)]
    pub file_upload: Option<Promise<Option<(String, String)>>>,
    pub is_open: bool,
}

impl FileHandler {
    pub fn new() -> Self {
        Self {
            dropped_files: Vec::<egui::DroppedFile>::default(),
            picked_path: None,
            file_upload: None,
            is_open: false,
        }
    }
    fn handle_dialog(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.file_upload = Some(Promise::spawn_local(async {
                let file = rfd::AsyncFileDialog::new().pick_file().await;
                if let Some(file) = file {
                    let buf = file.read().await;
                    return match std::str::from_utf8(&buf) {
                        Ok(v) => Some((v.to_string(), file.file_name())),
                        Err(e) => Some((e.to_string(), "".to_string())),
                    };
                }
                Some(("No file Selected".to_string(), "".to_string()))
            }));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.file_upload = Some(Promise::spawn_thread("slow", move || {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    // read file as string
                    if let Some(path) = path.to_str() {
                        let path = path.to_string();
                        let buf = std::fs::read(path.clone());
                        let buf = match buf {
                            Ok(v) => v,
                            Err(e) => {
                                log::warn!("{:?}", e);
                                return Some((e.to_string(), "".to_string()));
                            }
                        };
                        return match std::str::from_utf8(&buf) {
                            Ok(v) => {
                                return Some((v.to_string(), path));
                            }
                            Err(e) => Some((e.to_string(), "".to_string())),
                        };
                    }
                }
                Some(("No file Selected".to_string(), "".to_string()))
            }))
        }
    }
}

impl super::PanelController for FileHandler {
    fn name(&self) -> &'static str {
        "File Handler"
    }
    fn window_title(&self) -> &'static str {
        "File Handler"
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

impl super::PanelView for FileHandler {
    fn ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Open file…").clicked() {
            self.handle_dialog();
        }
        // Show dropped files (if any):
        if !self.dropped_files.is_empty() {
            ui.group(|ui| {
                ui.label("Dropped files:");

                for file in &self.dropped_files {
                    let mut info = if let Some(path) = &file.path {
                        path.display().to_string()
                    } else if !file.name.is_empty() {
                        file.name.clone()
                    } else {
                        "???".to_owned()
                    };

                    let mut additional_info = vec![];
                    if !file.mime.is_empty() {
                        additional_info.push(format!("type: {}", file.mime));
                    }
                    if let Some(bytes) = &file.bytes {
                        additional_info.push(format!("{} bytes", bytes.len()));
                    }
                    if !additional_info.is_empty() {
                        info += &format!(" ({})", additional_info.join(", "));
                    }

                    ui.label(info);
                }
            });
        }
        if self.picked_path.is_none() {
            if let Some(result) = self.file_upload.as_mut() {
                if let Some(ready) = result.ready() {
                    if let Some(file) = ready.clone() {
                        self.picked_path = Some(file.1);
                        // TODO
                    }
                }
            }
        }

        if let Some(picked_path) = &self.picked_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);
            });
        }

        // Collect dropped files:
        ui.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files = i.raw.dropped_files.clone();
            }
        });
    }
}
