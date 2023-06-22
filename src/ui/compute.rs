// [[file:../../bevy.note::83513dad][83513dad]]
use bevy::prelude::Resource;
use bevy_egui::egui;
use bevy_egui::EguiContext;
use egui::Ui;
use gut::prelude::*;
// 83513dad ends here

// [[file:../../bevy.note::52d286c4][52d286c4]]
#[derive(Debug, PartialEq, Deserialize, Default, Serialize)]
enum Code {
    #[default]
    Vasp,
    Gaussian,
    Orca,
}
// 52d286c4 ends here

// [[file:../../bevy.note::ba17983a][ba17983a]]
use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize, Serialize, Resource)]
pub struct State {
    code: Code,
    vasp_state: super::vasp::State,
    orca_state: super::orca::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            code: Code::default(),
            vasp_state: super::vasp::State::default(),
            orca_state: super::orca::State::default(),
        }
    }
}
// ba17983a ends here

// [[file:../../bevy.note::16a85587][16a85587]]
impl State {
    fn show_side_panel(&mut self, ui: &mut Ui) {
        ui.heading("Compute engines");
        ui.separator();

        ui.vertical(|ui| {
            ui.selectable_value(&mut self.code, Code::Vasp, "VASP");
            ui.selectable_value(&mut self.code, Code::Orca, "ORCA");
            ui.selectable_value(&mut self.code, Code::Gaussian, "Gaussian");
        });

        ui.separator();

        // show egui logo
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Find ");
                ui.hyperlink_to("more", "https://github.com/ybyygu/ui-hack");
            });
        });
    }
}
// 16a85587 ends here

// [[file:../../bevy.note::a6fccc52][a6fccc52]]
impl State {
    fn show_central_panel(&mut self, ui: &mut Ui) {
        ui.heading(format!("{:?} input generator", self.code));
        ui.separator();

        match self.code {
            Code::Vasp => {
                self.vasp_state.show(ui);
            }
            Code::Gaussian => {
                // 格线对齐
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Title:");
                        ui.end_row();
                        ui.label("Charge:");
                        ui.end_row();
                        ui.label("Multiplicity:");
                        ui.end_row();
                    });
            }
            Code::Orca => {
                self.orca_state.show(ui);
            }
            _ => {
                ui.label("Under Construction!");
            }
        }
    }
}
// a6fccc52 ends here

// [[file:../../bevy.note::39717a88][39717a88]]
impl State {
    /// Show UI for all orca settings
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            self.show_side_panel(ui);
        });

        // The central panel the region left after adding TopPanel's and SidePanel's
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_central_panel(ui);
        });
    }
}
// 39717a88 ends here
