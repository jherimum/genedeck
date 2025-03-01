use crate::sequence::Sequence;

#[allow(dead_code)]

pub struct Card {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Genes {
    alpha: Sequence,
    beta: Sequence,
    gamma: Sequence,
    delta: Sequence,
    epsilon: Sequence,
}
