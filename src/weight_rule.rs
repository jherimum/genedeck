use crate::protein::Protein;

static SEQUENCE_WEIGTH_RULES: &[Rule] = &[start_end_same_protein];
static GENE_WEIGTH_RULES: &[Rule] = &[start_end_same_protein];

pub type Rule = fn(&[Protein]) -> f32;

fn start_end_same_protein(sequence: &[Protein]) -> f32 {
    match (sequence.first(), sequence.last()) {
        (Some(start), Some(end)) if start == end => 1.1,
        _ => 1.0,
    }
}

pub fn calculate_sequence_weight(sequence: &[Protein]) -> f32 {
    SEQUENCE_WEIGTH_RULES
        .iter()
        .fold(0.0, |acc, rule| acc + rule(sequence))
}

pub fn calculate_gene_weight(sequence: &[Protein]) -> f32 {
    GENE_WEIGTH_RULES
        .iter()
        .fold(0.0, |acc, rule| acc + rule(sequence))
}
