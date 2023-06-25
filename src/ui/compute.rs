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
    Cp2k,
    Siesta,
    Lammps,
    Gulp,
    Rest,
}
// 52d286c4 ends here

// [[file:../../bevy.note::ba17983a][ba17983a]]
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct State {
    code: Code,
    vasp_state: super::vasp::State,
    orca_state: super::orca::State,
    gaussian_state: super::gaussian::State,
    cp2k_state: super::cp2k::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            code: Code::default(),
            vasp_state: super::vasp::State::default(),
            orca_state: super::orca::State::default(),
            gaussian_state: super::gaussian::State::default(),
            cp2k_state: super::cp2k::State::default(),
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
            ui.selectable_value(&mut self.code, Code::Cp2k, "CP2K");
            ui.selectable_value(&mut self.code, Code::Orca, "ORCA");
            ui.selectable_value(&mut self.code, Code::Gaussian, "Gaussian");
            ui.selectable_value(&mut self.code, Code::Rest, "REST");
            ui.selectable_value(&mut self.code, Code::Gulp, "GULP");
            ui.selectable_value(&mut self.code, Code::Siesta, "SIESTA");
            ui.selectable_value(&mut self.code, Code::Lammps, "LAMMPS");
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
    fn show_central_panel(&mut self, ui: &mut Ui, mol: Option<gchemol::Molecule>) {
        ui.heading(format!("{:?} input generator", self.code));
        ui.separator();

        match self.code {
            Code::Vasp => {
                self.vasp_state.show(ui);
            }
            Code::Cp2k => {
                self.cp2k_state.show(ui, mol);
            }
            Code::Orca => {
                self.orca_state.show(ui, mol);
            }
            Code::Gaussian => {
                self.gaussian_state.show(ui, mol);
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
    pub fn show(&mut self, ctx: &egui::Context, mol: Option<gchemol::Molecule>) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            self.show_side_panel(ui);
        });

        // The central panel the region left after adding TopPanel's and SidePanel's
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_central_panel(ui, mol);
        });
    }
}
// 39717a88 ends here
