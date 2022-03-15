use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|c| !['G', 'C', 'T', 'A'].contains(&c)) {
            Some(index) => Err(index),
            None => Ok(Dna(String::from(dna))),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.chars().fold(String::new(), |mut s, c| {
            s.push(
                *BTreeMap::from([('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')])
                    .get(&c)
                    .unwrap(),
            );
            s
        }))
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|c| !['C', 'G', 'A', 'U'].contains(&c)) {
            Some(index) => Err(index),
            None => Ok(Rna(String::from(rna))),
        }
    }
}
