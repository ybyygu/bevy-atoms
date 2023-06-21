// [[file:../../bevy.note::9c9c603b][9c9c603b]]
use bevy_egui::egui;
use egui::Ui;
use gut::prelude::*;
// 9c9c603b ends here

// [[file:../../bevy.note::7ae276e4][7ae276e4]]
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
struct Settings {
    // general
    encut: usize,
    // non-spin polarised DFT
    ispin: usize,

    // electronic relaxation
    ismear: usize,
    // smearing value in eV
    sigma: f64,
    // max electronic SCF steps
    nelm: usize,
    // min electronic SCF steps
    nelmin: usize,
    ediff: f64,

    // useful ionic relaxation
    nsw: usize,
    ibrion: usize,
    isif: usize,
    ediffg: f64,

    // symmetry
    isym: isize,

    // output verbosity
    nwrite: usize,
    // magnetization output
    lorbit: usize,
    lwave: bool,
    lcharg: bool,
}
// 7ae276e4 ends here

// [[file:../../bevy.note::d7d12e5e][d7d12e5e]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct State {
    settings: Settings,
    templates: Vec<String>,
    current_template: String,
    rendered_input: String,
    input_template: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            templates: vec!["normal.jinja".to_owned(), "spectrum.jinja".to_owned(), "custom".to_owned()],
            current_template: "normal.jinja".to_owned(),
            input_template: String::new(),
            rendered_input: String::new(),
        }
    }
}
// d7d12e5e ends here

// [[file:../../bevy.note::905e0f65][905e0f65]]
impl State {
    /// Show egui UI
    pub fn show(&mut self, ui: &mut Ui) {
        egui::Grid::new("vasp_grid_core").num_columns(2).show(ui, |ui| {
            ui.hyperlink_to("ENCUT", "https://www.vasp.at/wiki/index.php/ENCUT")
                .on_hover_text("specifies the cutoff energy for the plane-wave-basis set in eV");
            ui.add(egui::DragValue::new(&mut self.settings.encut).speed(10));
            ui.hyperlink_to("ISPIN", "https://www.vasp.at/wiki/index.php/ISPIN")
                .on_hover_text("ISPIN specifies spin polarization");
            ui.add(egui::DragValue::new(&mut self.settings.ispin).clamp_range(1..=2).speed(1));
            ui.end_row();
        });

        ui.collapsing("Electronic relaxation", |ui| {
            egui::Grid::new("vasp_grid_elec")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.hyperlink_to("EDIFF", "https://www.vasp.at/wiki/index.php/EDIFF")
                        .on_hover_text("EDIFF specifies the global break condition for the electronic SC-loop.");
                    ui.add(egui::DragValue::new(&mut self.settings.ediff).speed(1E-6));
                    ui.hyperlink_to("SIGMA", "https://www.vasp.at/wiki/index.php/SIGMA")
                        .on_hover_text("specifies the width of the smearing in eV");
                    ui.add(
                        egui::DragValue::new(&mut self.settings.sigma)
                            .clamp_range(0.0..=2.0)
                            .speed(0.1),
                    );
                    ui.end_row();
                    ui.hyperlink_to("NELM", "https://www.vasp.at/wiki/index.php/NELM")
                        .on_hover_text("sets the maximum number of electronic SC (self-consistency) steps.");
                    ui.add(egui::DragValue::new(&mut self.settings.nelm).speed(10));
                    ui.hyperlink_to("NELMIN", "https://www.vasp.at/wiki/index.php/NELMIN")
                        .on_hover_text("specifies the minimum number of electronic self-consistency steps. ");
                    ui.add(egui::DragValue::new(&mut self.settings.nelmin).speed(10));
                    ui.end_row();
                });
        });

        ui.collapsing("Ionic relaxation", |ui| {
            egui::Grid::new("vasp_grid_ionic")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.hyperlink_to("NSW", "https://www.vasp.at/wiki/index.php/NSW")
                        .on_hover_text("defines the maximum number of ionic steps");
                    ui.add(egui::DragValue::new(&mut self.settings.nsw).speed(1));
                    ui.hyperlink_to("EDIFFG", "https://www.vasp.at/wiki/index.php/EDIFFG")
                        .on_hover_text("defines the break condition for the ionic relaxation loop");
                    ui.add(egui::DragValue::new(&mut self.settings.ediffg).speed(1E-3));
                    ui.end_row();
                    ui.hyperlink_to("ISYM", "https://www.vasp.at/wiki/index.php/ISYM")
                        .on_hover_text("determines the way VASP treats symmetry");
                    ui.add(egui::DragValue::new(&mut self.settings.ediffg).clamp_range(-1..=3).speed(1));
                });
        });
    }
}
// 905e0f65 ends here
