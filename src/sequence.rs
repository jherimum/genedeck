use crate::{gene::Gene, protein::Protein, weight_rule::calculate_sequence_weight};
use arrayvec::ArrayVec;
use rand::Rng;
use std::fmt::Display;

fn even(value: usize) -> bool {
    value % 2 == 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sequence([Gene; 8]);

impl Sequence {
    /// Create a new sequence with 4 random genes and 4 default genes
    pub fn genesis() -> Self {
        Self(
            (0..8)
                .fold(ArrayVec::<Gene, 8>::new(), |mut acc, index| match index {
                    0..4 => {
                        acc.push(Gene::genesis());
                        acc
                    }
                    _ => {
                        acc.push(Gene::default());
                        acc
                    }
                })
                .into_inner()
                .unwrap(),
        )
    }

    pub fn new(genes: [Gene; 8]) -> Self {
        Self(genes)
    }

    pub fn value(&self) -> f32 {
        let value = self
            .0
            .iter()
            .enumerate()
            .fold(0.0, |acc, (index, gene)| match even(index) {
                true => acc + gene.value() as f32,
                false => acc - gene.value() as f32,
            });

        let protein_sequence: [Protein; 32] = self.into();
        let weigth = calculate_sequence_weight(&protein_sequence);

        value * weigth
    }

    /// Merge two sequences and mutate the last 4 genes
    pub fn merge(&self, other: &Sequence) -> Sequence {
        let mut sequence: ArrayVec<Gene, 8> = ArrayVec::new();
        for index in 0..8 {
            let mut gene = self.0[index].merge(&other.0[index]);
            if index > 3 {
                gene.mutate();
            }
            sequence.push(gene);
        }
        Self(sequence.into_inner().unwrap())
    }

    /// Mutate a random gene from the last 4 genes
    pub fn mutate(&mut self) {
        let index = rand::rng().random_range(5..8);
        self.0[index].mutate();
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self
            .0
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("-");

        write!(f, "{}", joined)
    }
}

impl From<&Sequence> for [Protein; 32] {
    fn from(value: &Sequence) -> Self {
        let mut a: [Protein; 32] = Default::default();
        let proteins = value.0.iter().fold(Vec::new(), |mut acc, gene| {
            acc.extend_from_slice(gene.as_ref());
            acc
        });

        a.copy_from_slice(&proteins[0..32]);
        a
    }
}

#[cfg(test)]
mod test {
    use crate::{gene::Gene, protein::Protein};

    #[test]
    fn test_display() {
        use super::Sequence;

        // let sequence = Sequence::new([
        //     Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]),
        //     Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]),
        //     Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]),
        //     Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]),
        //     Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]),
        //     Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]),
        //     Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]),
        //     Gene::new([Protein::A, Protein::A, Protein::A, Protein::A]),
        // ]);
        // assert_eq!(
        //     sequence.to_string(),
        //     "AAAA-AAAA-AAAA-AAAA-AAAA-AAAA-AAAA-AAAA"
        // );

        for _ in 0..100 {
            let sequence = Sequence::genesis();
            println!("{}", sequence);
        }
    }
}
