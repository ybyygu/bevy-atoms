// [[file:../../bevy.note::81ee36f4][81ee36f4]]
use bevy_egui::egui;
use egui::Ui;
use enum_iterator::Sequence;

use gchemol::Molecule;
use gut::prelude::*;
// 81ee36f4 ends here

// [[file:../../bevy.note::9c74c607][9c74c607]]
#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum Method {
    HF,
    PBE,
    BP86,
    TPSS,
    #[default]
    B3LYP,
    X3LYP,
    M06,
    M062X,
    wB97X,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum DFTGrid {
    Coarse,
    Fine,
    UltraFine,
    Superfine,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
// Symmetry=None
enum Symmetry {
    None,
    #[default]
    On,
    Loose,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize, Sequence)]
enum BasisSet {
    #[default]
    #[serde(rename = "6-31G**")]
    PopleDzp,
    #[serde(rename = "6-311+G**")]
    PopleTzpd,
    #[serde(rename = "def2SVP")]
    Def2Svp,
    #[serde(rename = "def2TZVP")]
    Def2Tzvp,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Multiplicity(usize);

impl Default for Multiplicity {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum ScfType {
    /// spin unrestricted SCF
    #[serde(rename = "U")]
    UnRestricted,

    /// closed-shell SCF
    #[serde(rename = "R")]
    Restricted,

    /// open-shell spin restricted SCF
    #[serde(rename = "RO")]
    RestrictedOpen,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum ScfOptions {
    VarAcc,
    Tight,
    Big,
    QC,
    XQC,
    YQC,
    Direct,
    NoDirect,
    InCore,
}

/// DFT Calculations with Atom-pairwise Dispersion Correction
/// https://gaussian.com/dft/?tabid=3
#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum Dispersion {
    /// Add the D2 version of Grimme’s dispersion
    GD2,
    /// Add the D3 version of Grimme’s dispersion with the original D3 damping function
    GD3,
    /// Add the D3 version of Grimme’s dispersion with Becke-Johnson damping
    GD3BJ,
    /// Add the Petersson-Frisch dispersion model from the APFD functional
    PFD,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum RIApproximation {
    RIJCOSX,
    NoCOSX,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Settings {
    method: Method,
    symmetry: Symmetry,
    dft_grid: Option<DFTGrid>,
    basis_set: BasisSet,
    charge: isize,
    multiplicity: Multiplicity,
    scf_type: Option<ScfType>,
    scf_convergence: Option<ScfOptions>,
    dispersion: Option<Dispersion>,
}
// 9c74c607 ends here

// [[file:../../bevy.note::a3be178b][a3be178b]]
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
            template_state: super::template::State::new("gaussian-templates".as_ref()),
        }
    }
}
// a3be178b ends here

// [[file:../../bevy.note::adbd1801][adbd1801]]
impl State {
    /// Show UI for Gaussian input generator
    pub fn show(&mut self, ui: &mut Ui, mol: Option<Molecule>) {
        egui::Grid::new("gaussian_grid_core").num_columns(2).show(ui, |ui| {
            // method
            ui.hyperlink_to("Method", "https://gaussian.com/dft");
            show_combo_box_enum!("gaussian-method", ui, self.settings.method, Method, 200.0);
            ui.end_row();
            // basis set
            ui.hyperlink_to("Basis set", "https://gaussian.com/basissets/");
            show_combo_box_enum!("gaussian-basis", ui, self.settings.basis_set, BasisSet, 200.0);
            ui.end_row();
            ui.label("Charge");
            ui.add(egui::DragValue::new(&mut self.settings.charge).speed(1.0));
            ui.label("Multiplicity");
            ui.add(
                egui::DragValue::new(&mut self.settings.multiplicity.0)
                    .clamp_range(1..=100)
                    .speed(1),
            );
        });

        ui.collapsing("SCF", |ui| {
            egui::Grid::new("gaussian_grid_scf")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    // SCF type
                    ui.hyperlink_to("SCF Type", "https://gaussian.com/scf/")
                        .on_hover_text("This keyword controls the functioning of the SCF procedure");
                    show_combo_box_enum!("gaussian-scf-type", ui, self.settings.scf_type, ScfType, 200.0);
                    // SCF convergence
                    ui.end_row();
                    ui.hyperlink_to("SCF Options", "https://gaussian.com/scf/?tabid=1")
                        .on_hover_text("This keyword controls the functioning of the SCF procedure");
                    show_combo_box_enum!("gaussian-scf-options", ui, self.settings.scf_convergence, ScfOptions, 200.0);
                    // DFT grid
                    ui.end_row();
                    ui.hyperlink_to("DFT Grid", "https://gaussian.com/integral/");
                    show_combo_box_enum!("gaussian-dft-grid", ui, self.settings.dft_grid, DFTGrid, 200.0);
                });
        });

        ui.collapsing("Misc", |ui| {
            egui::Grid::new("gaussian_grid_misc")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    // symmetry
                    ui.hyperlink_to("Symmetry", "https://gaussian.com/symmetry/")
                        .on_hover_text("This keyword specifies the uses of molecular symmetry within the calculation");
                    let radio = &mut self.settings.symmetry;
                    ui.horizontal(|ui| {
                        ui.selectable_value(radio, Symmetry::None, "None");
                        ui.selectable_value(radio, Symmetry::On, "On");
                        ui.selectable_value(radio, Symmetry::Loose, "Loose");
                    });
                    // dispersion
                    ui.end_row();
                    ui.hyperlink_to("Dispersion", "https://gaussian.com/dft/?tabid=3")
                        .on_hover_text("The EmpiricalDispersion keyword enables empirical dispersion.");
                    show_combo_box_enum!("gaussian-dispersion", ui, self.settings.dispersion, Dispersion, 200.0);
                })
        });

        ui.separator();
        // Gaussian input needs molecule data
        self.template_state.show_template_selection(&self.settings, ui, mol);
    }
}
// adbd1801 ends here
