use std::collections::HashMap;

use crate::person::Person;
use crate::relation::Relation;

pub struct Guests{
    pub people: HashMap<String, Person>,
    pub relations: Vec<Relation>,
}

impl Default for Guests{
    fn default() -> Self {
        Self { people: HashMap::new(), relations: Vec::new() }
    }
}

impl Guests {
    pub fn is_guest_known(&self, person_name: &str) -> bool
    {
        return self.people.iter().any(|c| c.0.eq(person_name))
    }

    pub fn is_person_seated(&self, person_name: &str) -> bool
    {
        match self.people.get(person_name)
        {
            Some(guest) => return guest.get_seated(),
            None => panic!("Searched for non-existing guest"),
        }
    }

    pub fn is_relation_known(&self, person1_name: &str, person2_name: &str) -> bool
    {
        return self.relations.iter().any(|r| r.person_in_relation(person1_name) && r.person_in_relation(person2_name));
    }

    pub fn seat_person(&self, person_name: &str)
    {
        match self.people.get(person_name)
        {
            Some(guest) => guest.seat_person(),
            None => {
                println!("Tried to seat non existing guest: {:?}", person_name);
                panic!();
            }
        }
    }

    pub fn unseat_guest(&self, person_name: &str)
    {
        match self.people.get(person_name)
        {
            Some(guest) => guest.reset_seating(),
            None => {
                println!("Tried to unseat non existing guest: {:?}", person_name);
                panic!();
            }
        }
    }

    pub fn get_amount_seated_guests(&self) -> u32
    {
        let mut visited: u32 = 0;
        for guest in &self.people
        {
            if guest.1.get_seated()
            {
                visited += 1;
            }
        }
        visited
    }
}