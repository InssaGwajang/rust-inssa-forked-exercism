use self::Allergen::*;

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}
impl Allergen {
    const VALUES: [Self; 8] = [
        Eggs,
        Peanuts,
        Shellfish,
        Strawberries,
        Tomatoes,
        Chocolate,
        Pollen,
        Cats,
    ];
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::VALUES
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .cloned()
            .collect()
    }
}

// pub struct Allergies {
//     allergies: Vec<Allergen>,
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum Allergen {
//     Eggs,
//     Peanuts,
//     Shellfish,
//     Strawberries,
//     Tomatoes,
//     Chocolate,
//     Pollen,
//     Cats,
//     Ignore,
// }

// fn test_allergen(score: u32) -> Option<(u32, Allergen)> {
//     if score == 0 {
//         return None;
//     }

//     let mut exp: u32 = 0;
//     while score >= 2u32.pow(exp + 1) {
//         exp += 1;
//     }
//     let index: usize = exp as usize;

//     let allergies: Vec<Allergen> = vec![
//         Allergen::Eggs,
//         Allergen::Peanuts,
//         Allergen::Shellfish,
//         Allergen::Strawberries,
//         Allergen::Tomatoes,
//         Allergen::Chocolate,
//         Allergen::Pollen,
//         Allergen::Cats,
//     ];

//     if index < allergies.len() {
//         Some((2u32.pow(exp), allergies[index]))
//     } else {
//         Some((2u32.pow(exp), Allergen::Ignore))
//     }
// }

// impl Allergies {
//     pub fn new(score: u32) -> Self {
//         let mut allergies: Vec<Allergen> = Vec::new();
//         let mut score = score;

//         while let Some((allergen_score, allergen)) = test_allergen(score) {
//             println!("{} {:?}", allergen_score, allergen);

//             score -= allergen_score;
//             if allergen != Allergen::Ignore {
//                 allergies.push(allergen);
//             }
//         }

//         Allergies {
//             allergies: allergies,
//         }
//     }

//     pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
//         self.allergies.contains(allergen)
//     }

//     pub fn allergies(&self) -> Vec<Allergen> {
//         self.allergies.clone()
//     }
// }
