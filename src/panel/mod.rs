pub mod file_handler;
pub mod logical_channels;
pub mod message;
pub mod panel;

pub use file_handler::FileHandler;
pub use logical_channels::LogicalChannels;
pub use message::MessageBox;
pub use panel::ConnectionPanel;

use eframe::egui;

/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait Demo {
    /// Is the demo enabled for this integration?
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
