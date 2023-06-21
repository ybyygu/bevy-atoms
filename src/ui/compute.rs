// [[file:../../bevy.note::83513dad][83513dad]]
use bevy_egui::egui;
use egui::Ui;
use gut::prelude::*;
// 83513dad ends here

// [[file:../../bevy.note::ba17983a][ba17983a]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct State {
    //
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}
// ba17983a ends here

// [[file:../../bevy.note::39717a88][39717a88]]
impl State {
    /// Show UI for all orca settings
    pub fn show(&mut self, ui: &mut Ui) {
        egui::Grid::new("orca_grid_core").num_columns(2).show(ui, |ui| {
            ui.label("Symmetry:");
            ui.end_row();
            ui.label("Symmetry:");
            ui.end_row();
            ui.label("Symmetry:");
            ui.end_row();
        });
        ui.collapsing("Misc", |ui| {
            egui::Grid::new("orca_grid_misc")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Symmetry:");
                });
        });
    }
}
// 39717a88 ends here
