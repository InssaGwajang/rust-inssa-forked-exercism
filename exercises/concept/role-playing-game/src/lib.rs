pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(Player {
                health: 100,
                mana: (self.level >= 10).then(|| 100),
                // mana: self.level.ge(&10).then(|| 100),
                // mana: if self.level >= 10 { Some(100) } else { None },
                ..*self
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana - mana_cost);
                mana_cost
                    .checked_mul(2)
                    .expect(format!("Damage is overflow from value:{}", mana_cost))
                // mana_cost * 2
            }
            Some(_) => 0,
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                // self.health = self.health - std::cmp::min(self.health, mana_cost);
                0
            }
        }
    }
}
