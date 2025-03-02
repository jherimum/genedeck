use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Protein {
    #[default]
    A,
    C,
    G,
    T,
}

impl Protein {
    pub fn random() -> Self {
        use rand::Rng;

        let mut x = rand::rng();
        match x.random_range(0..4) {
            0 => Protein::A,
            1 => Protein::C,
            2 => Protein::G,
            3 => Protein::T,
            _ => unreachable!(),
        }
    }

    pub fn random_except(except: Protein) -> Self {
        use rand::Rng;

        let protein: Protein = rand::rng().random_range(0..4).into();
        if except == protein {
            return Protein::random_except(protein);
        }

        return protein;
    }

    pub fn value(&self) -> u8 {
        self.into()
    }
}

impl Display for Protein {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Protein::A => 'A',
            Protein::T => 'T',
            Protein::G => 'G',
            Protein::C => 'C',
        };
        write!(f, "{}", c)
    }
}

impl From<u8> for Protein {
    fn from(value: u8) -> Self {
        match value {
            0 => Protein::A,
            1 => Protein::C,
            2 => Protein::G,
            3 => Protein::T,
            _ => unreachable!(),
        }
    }
}

impl Into<u8> for &Protein {
    fn into(self) -> u8 {
        match self {
            Protein::A => 0,
            Protein::T => 1,
            Protein::G => 2,
            Protein::C => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::Protein;

    #[test]
    fn random() {
        println!("{:?}", rand::rng().random_range(0..4));
        println!("{:?}", rand::rng().random_range(0..4));
        println!("{:?}", rand::rng().random_range(0..4));
        println!("{:?}", rand::rng().random_range(0..4));
        println!("{:?}", rand::rng().random_range(0..4));
        // let p1 = Protein::random();
        // let p2 = Protein::random();

        // println!("{:?}", p1);
        // println!("{:?}", p2);
    }
}
