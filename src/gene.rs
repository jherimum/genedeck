use crate::{protein::Protein, weight_rule::calculate_gene_weight};
use rand::Rng;
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug)]
pub struct Mutation {
    index: usize,
    old_protein: Protein,
    new_protein: Protein,
}

impl Mutation {
    pub fn new(index: usize, old_protein: Protein, new_protein: Protein) -> Self {
        Self {
            index,
            old_protein,
            new_protein,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn old_protein(&self) -> Protein {
        self.old_protein
    }

    pub fn new_protein(&self) -> Protein {
        self.new_protein
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gene([Protein; 4]);

impl Gene {
    pub fn genesis() -> Self {
        let proteins = [
            Protein::random(),
            Protein::random(),
            Protein::random(),
            Protein::random(),
        ];
        Self(proteins)
    }

    pub fn new(proteins: [Protein; 4]) -> Self {
        Self(proteins)
    }

    pub fn value(&self) -> f32 {
        let value: u64 = self.0.iter().map(|p| p.value() as u64).sum();
        let weight = calculate_gene_weight(&self.0);
        value as f32 * weight
    }

    pub fn mutate(&mut self) -> Mutation {
        let index = rand::rng().random_range(0..4);
        let actual_protein = self.0[index];
        let new_protein = Protein::random_except(actual_protein);
        self.0[index] = new_protein;
        Mutation::new(index, actual_protein, new_protein)
    }

    pub fn merge(&self, other: &Gene) -> Gene {
        for (x, y) in self.0.iter().zip(other.0.iter()) {
            match x.cmp(y) {
                Ordering::Greater => return self.clone(),
                Ordering::Less => return other.clone(),
                Ordering::Equal => continue,
            }
        }

        return self.clone();
    }
}

impl Default for Gene {
    fn default() -> Self {
        Self([Protein::A, Protein::A, Protein::A, Protein::A])
    }
}

impl AsRef<[Protein; 4]> for Gene {
    fn as_ref(&self) -> &[Protein; 4] {
        &self.0
    }
}

impl Display for Gene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for protein in self.0.iter() {
            write!(f, "{}", protein)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::protein::Protein;

    use super::Gene;

    #[test]
    fn test_display() {
        let gene = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        assert_eq!(gene.to_string(), "ATGC");

        let gene = Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]);
        assert_eq!(gene.to_string(), "AAAA");
    }

    #[test]
    fn test_as_ref() {
        let gene = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        assert_eq!(
            gene.as_ref(),
            &[Protein::A, Protein::T, Protein::G, Protein::C]
        );
    }

    #[test]
    fn test_default() {
        let gene = Gene::default();
        assert_eq!(
            gene,
            Gene::new([Protein::A, Protein::A, Protein::A, Protein::A])
        );
    }

    #[test]
    fn test_add() {
        let gene1 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        assert_eq!(gene1.merge(&gene2), gene1);

        let gene1 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::C, Protein::T, Protein::G, Protein::C]);
        assert_eq!(gene1.merge(&gene2), gene2);

        let gene1 = Gene::new([Protein::A, Protein::C, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::A, Protein::A, Protein::G, Protein::C]);
        assert_eq!(gene1.merge(&gene2), gene1);

        let gene1 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::T]);
        assert_eq!(gene1.merge(&gene2), gene2);

        let gene1 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::A]);
        assert_eq!(gene1.merge(&gene2), gene1);
    }

    #[test]
    fn test_mutate() {
        let gene = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let mut mutated = gene.clone();
        mutated.mutate();
        assert_ne!(gene, mutated);
    }
}
