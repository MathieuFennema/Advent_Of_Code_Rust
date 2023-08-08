use crate::component_traits::Component;
use crate::wire::Wire;

pub struct NGate
{
    input : String,
    output : String,
}

impl NGate
{
    pub fn new() -> Self
    {
        Self{input: String::new(), output: String::new()}
    }
}

impl Component for NGate
{
    fn add_input(&mut self, wire: &str)
    {
        self.input = wire.to_string();
    }

    fn get_input(&self) -> Vec<String>
    {
        vec![&self.input.to_string()]
    }   

    fn add_output(&mut self, wire: &str)
    {
        self.output = wire.to_string();
    }

    fn get_output(&self) -> String
    {
        &self.output.to_string()
    }
    
    fn validate_component(&self) -> bool
    {
        if !self.input.is_empty() && !self.output.is_empty()
        {
            return true;
        }
        false
    }

    fn compute_value(&mut self, wire_list: &mut Vec<Wire>)
    {
        let mut value : u16 = 0;
        if self.input.len() == 0
        {
            panic!("Left_shift with no inputs");
        }
        
        for wire in &mut *wire_list
        {
            if self.input == wire.get_name()
            {
                value = !wire.get_value().unwrap();
                break;
            }
        }
        
        for wire in wire_list
        {
            if wire.get_name() == self.output
            {
                wire.set_value(value);
                break;
            }
        }
    }
}

