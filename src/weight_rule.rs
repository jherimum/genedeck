use crate::protein::Protein;
use std::collections::HashSet;

static SEQUENCE_WEIGTH_RULES: &[Rule] = &[];

static GENE_WEIGTH_RULES: &[Rule] = &[];

pub type Rule = fn(&[Protein]) -> f32;

fn proteins_in_sequence(proteins: &[Protein]) -> f32 {
    fn find(proteins: &[Protein]) -> Option<&[Protein]> {
        let col_len = proteins.len();
        for pat_len in 1..=col_len / 2 {
            if col_len % pat_len == 0 {
                let pattern = &proteins[..pat_len];

                if proteins.chunks(pat_len).all(|chunk| chunk == pattern) {
                    return Some(pattern);
                }
            }
        }
        None
    }

    match find(proteins) {
        Some(pattern) if pattern.len() > 1 => 1.1,
        _ => 1.0,
    }
}

fn same_proteins(proteins: &[Protein]) -> f32 {
    let all_same = proteins.iter().all(|p| p == proteins.first().unwrap());
    if all_same { 1.1 } else { 1.0 }
}

fn start_end_same_protein(proteins: &[Protein]) -> f32 {
    match (proteins.first(), proteins.last()) {
        (Some(start), Some(end)) if start == end => 1.1,
        _ => 1.0,
    }
}

fn contain_all_proteins(proteins: &[Protein]) -> f32 {
    let all = proteins.iter().collect::<HashSet<_>>().len() == 4;
    if all { 1.1 } else { 1.0 }
}

pub fn calculate_sequence_weight(proteins: &[Protein]) -> f32 {
    // SEQUENCE_WEIGTH_RULES
    //     .iter()
    //     .fold(0.0, |acc, rule| acc + rule(proteins))
    1.0
}

pub fn calculate_gene_weight(proteins: &[Protein]) -> f32 {
    // GENE_WEIGTH_RULES
    //     .iter()
    //     .fold(0.0, |acc, rule| acc + rule(proteins))
    1.0
}

#[cfg(test)]
mod tests {
    use crate::{protein::Protein, weight_rule::proteins_in_sequence};

    #[test]
    fn proteins_in_sequence_test() {
        let proteins = vec![Protein::A, Protein::A, Protein::A, Protein::A];
        assert_eq!(proteins_in_sequence(&proteins), 1.1);
    }
}
