// [[file:../../bevy.note::9c9c603b][9c9c603b]]
#![deny(warnings)]
#![deny(clippy::all)]
#![allow(non_camel_case_types)]

use bevy_egui::egui;
use egui::Ui;
use gut::prelude::*;
// 9c9c603b ends here

// [[file:../../bevy.note::7ae276e4][7ae276e4]]
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
enum ISpin {
    #[default]
    #[serde(rename = "1")]
    NonSpinPolarized,
    #[serde(rename = "2")]
    SpinPolarized,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
/// ISIF determines whether the stress tensor is calculated
enum ISif {
    #[default]
    #[serde(rename = "2")]
    /// Relax ions with constant cell
    RelaxIons,
    /// Relax cell shape and volume
    #[serde(rename = "3")]
    RelaxCell,
    /// Relax cell shape but fix cell volume
    #[serde(rename = "4")]
    RelaxCellFixVolume,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
struct Settings {
    // general
    encut: usize,
    // spin polarised DFT?
    ispin: ISpin,

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
    isif: ISif,
    ediffg: f64,

    // symmetry
    isym: Option<isize>,

    // output verbosity
    nwrite: usize,
    lwave: bool,
    lcharg: bool,
    // write POSCAR or not
    write_poscar: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            // general
            encut: 400,
            // non-spin-polarised DFT
            ispin: ISpin::default(),

            // electronic relaxation
            ismear: 0,
            // smearing value in eV
            sigma: 0.05,
            // max electronic SCF steps
            nelm: 60,
            // min electronic SCF steps
            nelmin: 5,
            ediff: 1E-08,

            // useful ionic relaxation
            nsw: 100,
            ibrion: 2,
            isif: ISif::default(),
            ediffg: -0.01,

            // symmetry
            isym: None,

            // output verbosity
            nwrite: 2,
            lwave: false,
            lcharg: false,
            write_poscar: false,
        }
    }
}
// 7ae276e4 ends here

// [[file:../../bevy.note::d7d12e5e][d7d12e5e]]
#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    settings: Settings,
    template_state: super::template::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            template_state: super::template::State::new("vasp-templates".as_ref()),
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
            ui.add(egui::DragValue::new(&mut self.settings.encut).speed(10).suffix(" eV"));
            ui.hyperlink_to("ISPIN", "https://www.vasp.at/wiki/index.php/ISPIN")
                .on_hover_text("ISPIN specifies spin polarization");
            ui.selectable_value(&mut self.settings.ispin, ISpin::NonSpinPolarized, "non-spin-polarized");
            ui.selectable_value(&mut self.settings.ispin, ISpin::SpinPolarized, "spin-polarized");
        });

        ui.collapsing("Electronic relaxation", |ui| {
            egui::Grid::new("vasp_grid_elec")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.hyperlink_to("EDIFF", "https://www.vasp.at/wiki/index.php/EDIFF")
                        .on_hover_text("EDIFF specifies the global break condition for the electronic SC-loop.");
                    ui.add(
                        egui::DragValue::new(&mut self.settings.ediff)
                            .custom_formatter(|n, _| format!("{n:-8.0E}"))
                            .speed(0),
                    );
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
                    ui.add(egui::DragValue::new(&mut self.settings.ediffg).speed(0.01));
                    ui.hyperlink_to("ISYM", "https://www.vasp.at/wiki/index.php/ISYM")
                        .on_hover_text("determines the way VASP treats symmetry");
                    ui.selectable_value(&mut self.settings.isym, None, "Use Symmetry")
                        .on_hover_text(
                            "Switches on the use of symmetry. If selected, ISYM is not set and it will be automatically determined by VASP",
                        );
                    ui.selectable_value(&mut self.settings.isym, Some(0), "No Symmetry")
                        .on_hover_text("Switches off the use of symmetry. If selected, ISYM=0");
                    ui.end_row();
                    ui.hyperlink_to("ISIF", "https://www.vasp.at/wiki/index.php/ISIF")
                        .on_hover_text("determines whether the stress tensor is calculated");
                    ui.selectable_value(&mut self.settings.isif, ISif::RelaxIons, "Relax ions")
                        .on_hover_text("Relax ion positions with cell shape and volume fixed, corresponding to ISIF=2");
                    ui.selectable_value(&mut self.settings.isif, ISif::RelaxCell, "Relax cell")
                        .on_hover_text("Relax ion positions, cell shape and cell volume, corresponding to ISIF=3");
                    ui.selectable_value(&mut self.settings.isif, ISif::RelaxCellFixVolume, "Fix cell volume")
                        .on_hover_text("Relax ion positions and cell shape with fixed cell volume, corresponding to ISIF=4");
                });
        });

        ui.collapsing("Output control", |ui| {
            egui::Grid::new("vasp_grid_output")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.hyperlink_to("NWRITE", "https://www.vasp.at/wiki/index.php/NWRITE")
                        .on_hover_text("determines how much will be written to the file OUTCAR ('verbosity tag')");
                    ui.add(egui::DragValue::new(&mut self.settings.nwrite).clamp_range(0..=4).speed(1));
                    ui.hyperlink_to("LWAVE", "https://www.vasp.at/wiki/index.php/LWAVE")
                        .on_hover_text("determines whether the wavefunctions are written to the WAVECAR file");
                    ui.toggle_value(&mut self.settings.lwave, "write WAVECAR");
                    ui.hyperlink_to("LCHARG", "https://www.vasp.at/wiki/index.php/LCHARG")
                        .on_hover_text("determines whether the charge densities");
                    ui.toggle_value(&mut self.settings.lcharg, "write CHGCAR/CHG");
                    ui.end_row();
                    ui.checkbox(&mut self.settings.write_poscar, "write POSCAR file")
                        .on_hover_text("If checked, write POSCAR file based from current molecule geometry");
                });
        });
        ui.separator();

        self.template_state.show_template_selection(&self.settings, ui, None);
    }
}
// 905e0f65 ends here
