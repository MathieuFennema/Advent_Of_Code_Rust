use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
pub struct Recharge{
    name: String,
    lasts_turns: i32,
}

impl Effect for Recharge { 
    //get name of spell
    fn get_name(&self) -> String {
        self.name.clone()
    }
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        229
    }
    //get mana an effect gives
    fn get_mana(&self) -> i32 {
        101
    }
    //deduct rounds
    fn deduct_rounds_active(&mut self) {
        self.lasts_turns -= 1;
    }
    //Copy all values
    fn deep_copy_effect(&self) -> Box<dyn Effect>
    {
        Box::new(Recharge::new())
    }
}

impl Recharge {
    const NAME: &str = "Recharge";
    const LASTS_TURNS: i32 = 5;
    
    pub fn new() -> Self {
        Self{name: Self::NAME.to_string(), lasts_turns: Self::LASTS_TURNS}
    }
}