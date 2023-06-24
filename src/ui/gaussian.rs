// [[file:../../bevy.note::81ee36f4][81ee36f4]]
use bevy_egui::egui;
use egui::Ui;
use enum_iterator::Sequence;

use gchemol::Molecule;
use gut::prelude::*;
// 81ee36f4 ends here

// [[file:../../bevy.note::9c74c607][9c74c607]]
#[derive(Default, Debug, PartialEq, Deserialize, Serialize, Sequence)]
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
    current_template: String,
    rendered_input: String,
    input_template: String,
    /// User settings in json format
    json_input: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            current_template: "custom".to_owned(),
            input_template: String::new(),
            rendered_input: String::new(),
            json_input: String::new(),
        }
    }
}
// a3be178b ends here

// [[file:../../bevy.note::*macro/enum][macro/enum:1]]
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
// macro/enum:1 ends here

// [[file:../../bevy.note::fb4adf8c][fb4adf8c]]
use std::collections::HashMap;
use std::sync::OnceLock;

// the templates loaded from files
static TEMPLATES: OnceLock<HashMap<String, String>> = OnceLock::new();

fn render_template<S: Serialize>(template: &str, settings: S) -> Result<String> {
    let template = gchemol::io::Template::from_str(template);
    let s = template.render(settings)?;
    Ok(s)
}

impl State {
    fn templates() -> &'static HashMap<String, String> {
        let tpl_root_dir: &std::path::Path = "tests/files/gaussian-templates".as_ref();
        TEMPLATES.get_or_init(|| {
            let files = gchemol::io::find_files(".jinja", tpl_root_dir, true);
            let mut templates: HashMap<String, String> = files
                .map(|f| {
                    let tpl_key = f.strip_prefix(tpl_root_dir).unwrap().to_str().unwrap().to_owned();
                    let tpl_txt = gut::fs::read_file(f).unwrap();
                    (tpl_key, tpl_txt)
                })
                .collect();
            // allow user custom template
            templates.insert("custom".into(), String::new());
            info!("Loaded {} templates from {:?}", templates.len(), tpl_root_dir);
            templates
        })
    }

    fn show_template_selection(&mut self, ui: &mut Ui, mol: Option<Molecule>) {
        let templates = Self::templates();

        ui.horizontal(|ui| {
            ui.label("Render template:")
                .on_hover_text("Select predefined input templates. Swithc to `custom` for edit.");
            egui::ComboBox::from_id_source("vasp-template")
                .width(200.0)
                .selected_text(&self.current_template)
                .show_ui(ui, |ui| {
                    for t in templates.keys() {
                        ui.selectable_value(&mut self.current_template, t.to_string(), t);
                    }
                });
            // minijinja syntax reference
            ui.hyperlink_to(
                "Template Syntax Reference",
                "https://docs.rs/minijinja/latest/minijinja/syntax/index.html",
            );
        });
        // action button for render and copy to clipboard
        // write rendered input or the error when rendering
        let tooltip = "Click to copy generated input to clipboard";
        if ui.button("📋 Render & Copy").on_hover_text(tooltip).clicked() {
            let mut json_value = serde_json::to_value(&self.settings).ok();
            // append molecule object into user settings
            if json_value.is_some() {
                if let Some(json_object) = json_value.as_mut().unwrap().as_object_mut() {
                    if let Some(mol) = mol {
                        let mut mol_object = gchemol::io::to_json_value(&mol);
                        json_object.append(mol_object.as_object_mut().unwrap());
                    }
                    // println!("{}", serde_json::to_string_pretty(&json_object).unwrap());
                }
            }
            match render_template(&self.input_template, &json_value) {
                Ok(s) => {
                    self.rendered_input = s;
                }
                Err(e) => {
                    self.rendered_input = format!("minijinja template render issue:\n{e:?}");
                }
            }
            ui.output_mut(|o| o.copied_text = self.rendered_input.clone());
            // show json input for debug
            self.json_input = serde_json::to_string_pretty(&json_value.unwrap()).unwrap();
        }
        ui.separator();
        selectable_text(
            ui,
            &mut self.json_input.as_str(),
            "JSON input",
            "The json data used for rendering the template",
        );
        match self.current_template.as_str() {
            "custom" => {
                editable_text(ui, &mut self.input_template, "template");
            }
            t => {
                let mut s = templates[t].clone();
                selectable_text(
                    ui,
                    &mut s,
                    "template",
                    "Selected input template in minijinja format for rendering input file",
                );
                self.input_template = s.to_string();
            }
        }

        selectable_text(ui, &mut self.rendered_input.as_str(), "rendered", "Final input file");
    }
}

fn editable_text(ui: &mut Ui, text: &mut String, label: &str) {
    ui.collapsing(label, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(text)
                    .hint_text(label)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace.resolve(ui.style())),
            );
        });
    })
    .header_response
    .on_hover_text("You are free to edit the input template");
}

// NOTE: read-only
fn selectable_text(ui: &mut Ui, mut text: &str, label: &str, hint: &str) {
    ui.collapsing(label, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut text)
                    .hint_text(label)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace.resolve(ui.style())),
            );
        });
    })
    .header_response
    .on_hover_text(hint);
}
// fb4adf8c ends here

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
        self.show_template_selection(ui, mol);
    }
}
// adbd1801 ends here
