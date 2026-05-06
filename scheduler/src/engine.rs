use crate::models::ExperimentEntry;

pub fn ucb_score(exp: &ExperimentEntry, total_pulls: u32, c: f64) -> f64 {
    let exploration = c * ((total_pulls as f64).ln() / (exp.pulls as f64 + 1.0)).sqrt();
    exp.score + exploration
}

pub fn allocate(experiments: &[ExperimentEntry], gpu_count: usize, c: f64) -> Vec<String> {
    let total_pulls: u32 = experiments.iter().map(|e| e.pulls).sum();
    
    let mut scored: Vec<(&ExperimentEntry, f64)> = experiments
        .iter()
        .map(|e| (e, ucb_score(e, total_pulls, c)))
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scored.iter().take(gpu_count).map(|(e, _)| e.id.clone()).collect()
    }