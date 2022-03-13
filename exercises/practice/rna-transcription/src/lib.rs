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

// fn validate(s: &str, chars: [char; 4]) -> Result<String, usize> {
//     match s.chars().position(|c| !chars.contains(&c)) {
//         Some(x) => Err(x),
//         None => Ok(s.to_string()),
//     }
// }
// fn transcribe(nucleotide: char) -> char {
//     RNA[DNA.iter().position(|&c| c == nucleotide).unwrap()]
// }
// impl Dna {
//     pub fn new(dna: &str) -> Result<Dna, usize> {
//         validate(dna, DNA).map(|nucleotides| Dna { nucleotides })
//     }
//     pub fn into_rna(self) -> Rna {
//         Rna {
//             nucleotides: self.nucleotides.chars().map(|c| transcribe(c)).collect(),
//         }
//     }
// }
// impl Rna {
//     pub fn new(rna: &str) -> Result<Rna, usize> {
//         validate(rna, RNA).map(|nucleotides| Rna { nucleotides })
//     }
// }

// #[derive(Debug, PartialEq)]
// pub struct Dna {
//     nucleotides: Vec<DnaNucleotide>,
// }

// #[derive(Debug, PartialEq)]
// pub struct Rna {
//     nucleotides: Vec<RnaNucleotide>,
// }

// #[derive(Debug, PartialEq)]
// enum DnaNucleotide {
//     G,
//     C,
//     T,
//     A,
// }

// #[derive(Debug, PartialEq)]
// enum RnaNucleotide {
//     C,
//     G,
//     A,
//     U,
// }

// impl DnaNucleotide {
//     fn from(c: char) -> Option<DnaNucleotide> {
//         match c {
//             'G' => Some(DnaNucleotide::G),
//             'C' => Some(DnaNucleotide::C),
//             'T' => Some(DnaNucleotide::T),
//             'A' => Some(DnaNucleotide::A),
//             _ => None,
//         }
//     }

//     fn into(self) -> RnaNucleotide {
//         match self {
//             DnaNucleotide::G => RnaNucleotide::C,
//             DnaNucleotide::C => RnaNucleotide::G,
//             DnaNucleotide::T => RnaNucleotide::A,
//             DnaNucleotide::A => RnaNucleotide::U,
//         }
//     }
// }

// impl RnaNucleotide {
//     fn from(c: char) -> Option<RnaNucleotide> {
//         match c {
//             'C' => Some(RnaNucleotide::C),
//             'G' => Some(RnaNucleotide::G),
//             'A' => Some(RnaNucleotide::A),
//             'U' => Some(RnaNucleotide::U),
//             _ => None,
//         }
//     }
// }

// impl Dna {
//     pub fn new(dna: &str) -> Result<Dna, usize> {
//         let mut nucleotides: Vec<DnaNucleotide> = Vec::new();
//         for (index, c) in dna.chars().enumerate() {
//             match DnaNucleotide::from(c) {
//                 Some(nucleotide) => nucleotides.push(nucleotide),
//                 None => return Err(index),
//             }
//         }
//         Ok(Dna {
//             nucleotides: nucleotides,
//         })
//     }

//     pub fn into_rna(self) -> Rna {
//         Rna {
//             nucleotides: self.nucleotides.into_iter().fold(
//                 Vec::<RnaNucleotide>::new(),
//                 |mut nucleotides, nucleotide| {
//                     nucleotides.push(nucleotide.into());
//                     nucleotides
//                 },
//             ),
//         }
//     }
// }

// impl Rna {
//     pub fn new(rna: &str) -> Result<Rna, usize> {
//         let mut nucleotides: Vec<RnaNucleotide> = Vec::new();
//         for (index, c) in rna.chars().enumerate() {
//             match RnaNucleotide::from(c) {
//                 Some(nucleotide) => nucleotides.push(nucleotide),
//                 None => return Err(index),
//             }
//         }
//         Ok(Rna {
//             nucleotides: nucleotides,
//         })
//     }
// }
