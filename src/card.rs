use crate::sequence::Sequence;

pub struct Card {
    alpha: Sequence,
    beta: Sequence,
    gamma: Sequence,
    delta: Sequence,
    epsilon: Sequence,
}

impl Card {
    pub fn genesis() -> Self {
        Self {
            alpha: Sequence::genesis(),
            beta: Sequence::genesis(),
            gamma: Sequence::genesis(),
            delta: Sequence::genesis(),
            epsilon: Sequence::genesis(),
        }
    }

    pub fn new(
        alpha: Sequence,
        beta: Sequence,
        gamma: Sequence,
        delta: Sequence,
        epsilon: Sequence,
    ) -> Self {
        Self {
            alpha,
            beta,
            gamma,
            delta,
            epsilon,
        }
    }

    pub fn merge(&self, other: &Card) -> Card {
        Self {
            alpha: self.alpha.merge(&other.alpha),
            beta: self.beta.merge(&other.beta),
            gamma: self.gamma.merge(&other.gamma),
            delta: self.delta.merge(&other.delta),
            epsilon: self.epsilon.merge(&other.epsilon),
        }
    }
}
