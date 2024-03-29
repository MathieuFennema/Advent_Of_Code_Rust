use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
pub struct MagicMissile {
}

impl Effect for MagicMissile {
    //get name of spell
    fn get_name(&self) -> String {
        Self::NAME.to_string()
    }
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        Self::COST
    }
    //get damage an effect does
    fn get_dmg(&self) -> i32 {
        Self::DMG
    }
    //deduct rounds, but has no active rounds
    fn deduct_rounds_active(&mut self) {
        
    }
    //Copy all values
    fn deep_copy_effect(&self) -> Box<dyn Effect>
    {
        Box::new(MagicMissile::new())
    }
}

impl MagicMissile {
    pub const NAME: &'static str = "Magic Missile";
    const COST: i32 = 53;
    const DMG: i32 = 4;
    pub fn new() -> Self {
        Self{}
    }
}