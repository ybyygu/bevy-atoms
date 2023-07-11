// [[file:../../bevy.note::46a7bd1f][46a7bd1f]]
#![deny(warnings)]
#![deny(clippy::all)]
#![allow(non_camel_case_types)]

use bevy_egui::egui;
use egui::Ui;
use enum_iterator::Sequence;

use gchemol::Molecule;
use gut::prelude::*;
// 46a7bd1f ends here

// [[file:../../bevy.note::7ae276e4][7ae276e4]]
#[derive(Default, Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum Method {
    PBE,
    BP86,
    TPSS,
    #[default]
    B3LYP,
    X3LYP,
    PBE0,
    TPSSh,
    M06,
    M062X,
    wB97X,
    #[serde(rename = "wB97X-D3")]
    wB97XD3,
    /// Perdew’s SCAN functional
    #[serde(rename = "SCANfunc")]
    SCAN,
    MP2,
    CCSD,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum DFTGrid {
    DefGrid1,
    DefGrid2,
    DefGrid3,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
enum Symmetry {
    #[default]
    NoUseSym,
    UseSym,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize, Sequence)]
enum BasisSet {
    #[default]
    #[serde(rename = "def2-SVP")]
    Def2Svp,
    #[serde(rename = "def2-TZVP")]
    Def2Tzvp,
    #[serde(rename = "def2-TZVPP")]
    Def2Tzvpp,
    #[serde(rename = "def2-QZVPP")]
    Def2Qzvpp,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Multiplicity(usize);

impl Default for Multiplicity {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum SCFType {
    /// spin unrestricted SCF
    #[serde(rename = "UKS")]
    UnRestricted,

    /// closed-shell SCF
    #[serde(rename = "KS")]
    Restricted,

    /// open-shell spin restricted SCF
    #[serde(rename = "ROKS")]
    RestrictedOpen,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum SCFConvergence {
    // Energy change 1.0e-09 au
    VeryTightSCF,
    // Energy change 1.0e-08 au. Default for geometry optimizations.
    TightSCF,
    // Energy change 1.0e-06 au. Default for single-point calculations.
    NormalSCF,
    // Energy change 1.0e-05 au
    LooseSCF,
}

/// DFT Calculations with Atom-pairwise Dispersion Correction
#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum Dispersion {
    D2,
    D3BJ,
    D3Zero,
    // NOTE: there are bugs before version 5.0.4
    D4,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum RIApproximation {
    RIJCOSX,
    NoCOSX,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
struct Settings {
    method: Method,
    symmetry: Symmetry,
    dft_grid: Option<DFTGrid>,
    basis_set: BasisSet,
    // put a different basis set on a specific atom
    extra_basis_set: Option<BasisSet>,
    extra_basis_set_atoms: Vec<usize>,
    #[serde(skip)]
    _extra_basis_set_atoms: String,
    // The real string for ORCA. ORCA atom counting starts from 0.
    oniom_qm_atoms: String,
    #[serde(skip)]
    _oniom_qm_atoms: String,
    charge: isize,
    multiplicity: Multiplicity,
    scf_type: Option<SCFType>,
    scf_convergence: Option<SCFConvergence>,
    dispersion: Option<Dispersion>,
    ri: Option<RIApproximation>,
    // write molecule geometry in generated input
    write_molecule_geometry: bool,
}
// 7ae276e4 ends here

// [[file:../../bevy.note::52c8c6b5][52c8c6b5]]
#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    settings: Settings,
    // state for template rendering UI
    template_state: super::template::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            template_state: super::template::State::new("orca-templates".as_ref()),
        }
    }
}
// 52c8c6b5 ends here

// [[file:../../bevy.note::bc270427][bc270427]]
impl State {
    /// Show egui UI
    pub fn show(&mut self, ui: &mut Ui, mol: Option<&Molecule>) {
        egui::Grid::new("orca_grid_core").num_columns(2).show(ui, |ui| {
            // method
            ui.hyperlink_to("Method", "https://sites.google.com/site/orcainputlibrary/dft-calculations");
            show_combo_box_enum!("orca-method", ui, self.settings.method, Method, 200.0);
            ui.end_row();
            // basis set
            ui.hyperlink_to("Basis set", "https://sites.google.com/site/orcainputlibrary/basis-sets");
            show_combo_box_enum!("orca-basis", ui, self.settings.basis_set, BasisSet, 200.0);
            ui.end_row();
            ui.label("Charge");
            ui.add(egui::DragValue::new(&mut self.settings.charge).speed(1.0));
            ui.label("Multiplicity");
            ui.add(egui::DragValue::new(&mut self.settings.multiplicity.0).speed(1.0));
        });

        ui.collapsing("SCF", |ui| {
            egui::Grid::new("orca_grid_scf")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    // SCF type
                    ui.label("SCF Type");
                    show_combo_box_enum!("orca-scf-type", ui, self.settings.scf_type, SCFType, 200.0);
                    // SCF convergence
                    ui.end_row();
                    ui.label("SCF Convergence");
                    show_combo_box_enum!(
                        "orca-scf-convergence",
                        ui,
                        self.settings.scf_convergence,
                        SCFConvergence,
                        200.0
                    );
                    // DFT grid
                    ui.end_row();
                    ui.label("DFT Grid");
                    show_combo_box_enum!("orca-dft-grid", ui, self.settings.dft_grid, DFTGrid, 200.0);
                });
        });
        ui.collapsing("Multiscale", |ui| {
            egui::Grid::new("orca_grid_multiscale")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    // allow user set extra basis set for some atoms
                    ui.hyperlink_to("Extra basis set", "https://sites.google.com/site/orcainputlibrary/basis-sets");
                    show_combo_box_enum!("orca-extra-basis", ui, self.settings.extra_basis_set, BasisSet, 200.0);
                    ui.label("for atoms:")
                        .on_hover_text("put a different basis set on a specific atom");
                    if ui
                        .add(egui::TextEdit::singleline(&mut self.settings._extra_basis_set_atoms).clip_text(false))
                        .on_hover_text("for example: 1,5,8-10,12")
                        .lost_focus()
                    {
                        if !self.settings._extra_basis_set_atoms.is_empty() {
                            if let Ok(s) = gut::utils::parse_numbers_human_readable(&self.settings._extra_basis_set_atoms) {
                                if let Ok(t) = gut::utils::abbreviate_numbers_human_readable(&s) {
                                    self.settings._extra_basis_set_atoms = t;
                                    self.settings.extra_basis_set_atoms = s;
                                    // require write geometry
                                    self.settings.write_molecule_geometry = true;
                                }
                            }
                        }
                    }
                    ui.end_row();
                    // ONIOM

                    ui.hyperlink_to(
                        "ONIOM QM layer atom",
                        "https://www.orcasoftware.de/tutorials_orca/multi/basics-otheroniom.html",
                    )
                    .on_hover_text("Enable multiscale ONIOM method");
                    if ui
                        .add(egui::TextEdit::singleline(&mut self.settings._oniom_qm_atoms).clip_text(false))
                        .on_hover_text("The serial numbers of atoms in QM layer. For example: 1,5,8-10,12")
                        .lost_focus()
                    {
                        if !self.settings._oniom_qm_atoms.is_empty() {
                            if let Ok(qm_atoms) = gut::utils::parse_numbers_human_readable(&self.settings._oniom_qm_atoms) {
                                // ORCA counting atoms starts from 0
                                let mut qm_atoms_orca = qm_atoms.clone();
                                for i in 0..qm_atoms_orca.len() {
                                    qm_atoms_orca[i] -= 1;
                                }
                                if let Ok(t) = gut::utils::abbreviate_numbers_human_readable(&qm_atoms) {
                                    self.settings._oniom_qm_atoms = t;
                                    let t = gut::utils::abbreviate_numbers_human_readable(&qm_atoms_orca)
                                        .unwrap()
                                        .replace(",", " ")
                                        .replace("-", ":");
                                    self.settings.oniom_qm_atoms = t;
                                }
                            }
                        }
                    }
                });
        });

        ui.collapsing("Misc", |ui| {
            egui::Grid::new("orca_grid_misc")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    // symmetry
                    ui.label("Symmetry");
                    let radio = &mut self.settings.symmetry;
                    ui.horizontal(|ui| {
                        ui.selectable_value(radio, Symmetry::NoUseSym, "NoUseSym");
                        ui.selectable_value(radio, Symmetry::UseSym, "UseSym");
                    });
                    // dispersion
                    ui.end_row();
                    ui.label("Dispersion");
                    show_combo_box_enum!("orca-dispersion", ui, self.settings.dispersion, Dispersion, 200.0);
                    ui.end_row();
                    // RI
                    ui.label("RI Approximation");
                    show_combo_box_enum!("orca-ri", ui, self.settings.ri, RIApproximation, 200.0);
                    ui.end_row();
                    ui
                        .checkbox(&mut self.settings.write_molecule_geometry, "Write geometry")
                        .on_hover_text("If checked, write molecule geometry using `xyz` keyword. If not, read from a default xyzfile `orca-input.xyz`");
                    ui.end_row();
                })
        });

        ui.separator();
        self.template_state.show_template_selection(&self.settings, ui, mol);
    }
}
// bc270427 ends here
