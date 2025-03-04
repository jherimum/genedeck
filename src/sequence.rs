use crate::{gene::Gene, protein::Protein, weight_rule::calculate_sequence_weight};
use arrayvec::ArrayVec;
use rand::Rng;
use std::fmt::Display;

fn even(value: usize) -> bool {
    value % 2 == 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sequence([Gene; 8]);

impl Into<Vec<Protein>> for &Sequence {
    fn into(self) -> Vec<Protein> {
        self.0.iter().fold(Vec::new(), |mut acc, gene| {
            acc.extend_from_slice(gene.as_ref());
            acc
        })
    }
}

impl Sequence {
    /// Create a new sequence with 4 random genes and 4 default genes
    pub fn genesis() -> Self {
        let head = [
            Gene::genesis(),
            Gene::genesis(),
            Gene::genesis(),
            Gene::genesis(),
        ];
        let tail = [Gene::default(); 4];
        let concat: ArrayVec<Gene, 8> = head.into_iter().chain(tail.into_iter()).collect();
        Self(concat.into_inner().unwrap())
    }

    pub fn new(genes: [Gene; 8]) -> Self {
        Self(genes)
    }

    pub fn value(&self) -> u64 {
        let value: u64 = self.0.iter().map(Gene::value).sum();
        let proteins: Vec<Protein> = self.into();
        let weigth = calculate_sequence_weight(&proteins);
        (value as f32 * weigth).trunc() as u64
    }

    /// Merge two sequences and mutate the last 4 genes
    pub fn merge(&self, other: &Sequence) -> Sequence {
        let mut genes = [Gene::default(); 8];
        for index in 0..8 {
            let mut gene = self.0[index].merge(&other.0[index], even(index));
            if index > 3 {
                gene.mutate();
            }
            genes[index] = gene;
        }

        Self(genes)
    }

    /// Mutate a random gene from the last 4 genes
    pub fn mutate(&mut self) {
        let index = rand::rng().random_range(0..8);
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

        for i in 0..10 {
            let mut sequence = Sequence::genesis();
            println!("sequence-{} {}: {}", i, sequence, sequence.value());
        }
    }
}
