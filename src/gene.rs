use crate::protein::Protein;
use rand::Rng;
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gene([Protein; 4]);

impl Gene {
    pub fn genesis() -> Self {
        Self([
            Protein::random(),
            Protein::random(),
            Protein::random(),
            Protein::random(),
        ])
    }

    pub fn new(proteins: [Protein; 4]) -> Self {
        Self(proteins)
    }

    pub fn value(&self) -> u64 {
        let value: u64 = self
            .0
            .iter()
            .enumerate()
            .map(|(ix, protein)| ((ix + 1) as u8 * protein.value()) as u64)
            .sum();
        let weight = 1.1; // calculate_gene_weight(&self.proteins());
        (value as f32 * weight).trunc() as u64
    }

    fn proteins(&self) -> Vec<Protein> {
        self.0.iter().copied().collect()
    }

    pub fn mutate(&mut self) {
        let index = rand::rng().random_range(0..4);
        self.0[index] = Protein::random();
    }

    pub fn merge(&self, other: &Gene, asc: bool) -> Gene {
        for (x, y) in self.0.iter().zip(other.0.iter()) {
            match (x.cmp(y), asc) {
                (Ordering::Greater, true) | (Ordering::Less, false) => self.clone(),
                (Ordering::Less, true) | (Ordering::Greater, false) => other.clone(),
                (Ordering::Equal, _) => continue,
            };
        }

        return self.clone();
    }
}

impl Default for Gene {
    fn default() -> Self {
        Self([Protein::T, Protein::G, Protein::C, Protein::A])
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
        write!(f, "::{} ", self.value())?;
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
        let gene1 = Gene::new([Protein::G, Protein::G, Protein::G, Protein::T]);
        let gene2 = Gene::new([Protein::T, Protein::G, Protein::T, Protein::C]);
        assert_eq!(gene1.merge(&gene2, true), gene1);

        let gene1 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::C, Protein::T, Protein::G, Protein::C]);
        assert_eq!(gene1.merge(&gene2, true), gene2);

        let gene1 = Gene::new([Protein::A, Protein::C, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::A, Protein::A, Protein::G, Protein::C]);
        assert_eq!(gene1.merge(&gene2, true), gene1);

        let gene1 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::T]);
        assert_eq!(gene1.merge(&gene2, true), gene2);

        let gene1 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let gene2 = Gene::new([Protein::A, Protein::T, Protein::G, Protein::A]);
        assert_eq!(gene1.merge(&gene2, true), gene1);
    }

    #[test]
    fn test_mutate() {
        let gene = Gene::new([Protein::A, Protein::T, Protein::G, Protein::C]);
        let mut mutated = gene.clone();
        mutated.mutate();
        assert_ne!(gene, mutated);
    }
}
