use std::cell::RefCell;
use std::rc::Rc;

use crate::component_traits::Component;

#[derive(Clone)]
pub struct OrGate
{
    input : Vec<Rc<RefCell<dyn Component>>>,
    value : Option<u16>,
}

impl OrGate
{
    pub fn new() -> Self
    {
        Self{input : vec![], value : None}
    }
}

impl Component for OrGate
{
    fn add_input(&mut self, input_comp: &Rc<RefCell<dyn Component>>)
    {
        self.input.push(input_comp.clone());
    }

    fn validate_component(&self) -> bool
    {
        if self.input.len() >= 2
        {
            return true;
        }
        false
    }

    fn compute_value(&mut self) -> u16
    {
        if self.value.is_none()
        {
            let mut return_val : u16 = 0;
            for component in &self.input
            {
                return_val |= component.borrow_mut().compute_value();
            }
            self.value = Some(return_val);
        }
        self.value.unwrap()
    }
}
