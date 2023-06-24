// [[file:../../bevy.note::46a7bd1f][46a7bd1f]]
use bevy_egui::egui;
use egui::Ui;
use enum_iterator::Sequence;

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
    charge: isize,
    multiplicity: Multiplicity,
    scf_type: Option<SCFType>,
    scf_convergence: Option<SCFConvergence>,
    dispersion: Option<Dispersion>,
    ri: Option<RIApproximation>,
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
            template_state: super::template::State::new("tests/files/orca-templates".as_ref()),
        }
    }
}
// 52c8c6b5 ends here

// [[file:../../bevy.note::45bd6a9d][45bd6a9d]]
macro_rules! enum_value {
    ($v:expr) => {{
        serde_json::to_string($v).unwrap().trim_matches('"').to_string()
    }};
}

macro_rules! show_combo_box_enum {
    ($id:literal, $ui:ident, $var:expr, $type:ty, $width:literal) => {
        let s = enum_value!(&$var);
        egui::ComboBox::from_id_source($id)
            .width($width)
            .selected_text(s)
            .show_ui($ui, |ui| {
                for t in enum_iterator::all::<$type>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut $var, t.into(), s);
                }
            });
    };
}
// 45bd6a9d ends here

// [[file:../../bevy.note::bc270427][bc270427]]
impl State {
    /// Show egui UI
    pub fn show(&mut self, ui: &mut Ui) {
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
                    // RI
                    ui.end_row();
                    ui.label("RI Approximation");
                    show_combo_box_enum!("orca-ri", ui, self.settings.ri, RIApproximation, 200.0);
                })
        });

        ui.separator();
        self.template_state.show_template_selection(&self.settings, ui, None);
    }
}
// bc270427 ends here
