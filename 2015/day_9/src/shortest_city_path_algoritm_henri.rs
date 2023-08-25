use std::rc::Rc;
use crate::country::Country;
use crate::city::City;
use crate::province::Province;
use crate::route::Route;

pub fn resolve_traveler_problem(country: Rc<Country>) -> Option<u32>
{
    let mut routes_taken: Vec<Route> = Vec::new();
    let mut cities_visited: Vec<String> = Vec::new();
    let mut provinces: Vec<Province> = Vec::new();
    let mut province_id_gen: u32 = 0;
    let mut shortest_path_all_cities: Option<u32> = None;


    /*kortste wegen gebruiken sorteren, steden koppelen aan provincies */
    /* meer koppels maken */
    /* kortste wegen tussen koppels & nieuwe stad, tot hele stad is betrokken*/
    let shortest_roads;//TODO


    // for city in &country.cities
    // {//take every city as starting point
    //     let mut shortest_path_this_city: u32 = u32::MAX;
    //     let mut amount_of_cities_connected: u32 = 1; // start with current city
    //     let mut already_traveled: Vec<String> = Vec::new();
    //     //*Magic*//
        
    //     //grab city 

    //     //see if we have connected all cities
    //     if amount_of_cities_connected == country.cities.len() as u32
    //     {
    //         match shortest_path_all_cities
    //         {
    //             Some(value) => {
    //                 if shortest_path_this_city < value
    //                 {
    //                     shortest_path_all_cities = Some(shortest_path_this_city);
    //                 }
    //             },
    //             None =>shortest_path_all_cities = Some(shortest_path_this_city),
    //         } 
    //     }
    // }
    shortest_path_all_cities
}
