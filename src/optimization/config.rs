use serde::{Deserialize, Serialize};

/// Contains all the configurable parameters of the algorithm

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub avg_nodes_removed: usize,
    pub blink_rate: f32,
    pub max_run_time: Option<f64>,
    #[serde(rename = "maxRRIterations")]
    pub max_rr_iterations: Option<usize>,
    pub leftover_valuation_power: f32,
    pub history_length: usize,
    pub rotation_allowed: bool,
    pub n_threads: usize,
    pub sheet_valuation_mode : SheetValuationMode,
    pub max_stages: Option<u8>,
    pub terminate_after_find_complete_solution: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SheetValuationMode {
    Area,
    Cost
}