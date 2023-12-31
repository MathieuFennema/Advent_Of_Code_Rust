use crate::translation_range::*;
use crate::plant_data::PlantData;

pub fn parse_data_from_file_content(file_content: &str, seeds_list: &mut Vec<PlantData>) -> Vec<TranslationTable>
{
    let mut temp : TranslationTable = TranslationTable { translation_step: TranslationStep::SeedToSoil, list: Vec::new() };
    let mut return_value: Vec<TranslationTable> = Vec::new();
    let mut current_map: &mut TranslationTable = &mut temp;

    for line in file_content.lines()
    {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty()
        {
            continue;
        }
        if line.starts_with("seeds:")
        {
            let mut selected_start_nr: bool = true;
            let mut start_nr: u64 = 0;
            for number_str in line.split_whitespace()
            {
                if selected_start_nr 
                {
                    match number_str.parse::<u64>()
                    {
                        Ok(seed_nr) => start_nr = seed_nr,
                        Err(_) => continue,
                    }
                    selected_start_nr = false;
                }
                else {
                    match number_str.parse::<u64>()
                    {
                        Ok(range) => seeds_list.push(PlantData::new(start_nr..(start_nr + range))),
                        Err(_) => continue,
                    }
                    selected_start_nr = true;
                }
                
            }
            continue;
        }
        if line.starts_with("seed-to-soil")
        {
            let trans_table = TranslationTable{translation_step: TranslationStep::SeedToSoil, list:Vec::new()};
            return_value.push(trans_table);
            current_map = return_value.last_mut().unwrap();
            continue;
        }
        if line.starts_with("soil-to-fertilizer")
        {
            let trans_table = TranslationTable{translation_step: TranslationStep::SoilToFertilizer, list:Vec::new()};
            return_value.push(trans_table);
            current_map = return_value.last_mut().unwrap();
            continue;
        }
        if line.starts_with("fertilizer-to-water")
        {
            let trans_table = TranslationTable{translation_step: TranslationStep::FertilizerToWater, list:Vec::new()};
            return_value.push(trans_table);
            current_map = return_value.last_mut().unwrap();
            continue;
        }
        if line.starts_with("water-to-light")
        {
            let trans_table = TranslationTable{translation_step: TranslationStep::WaterToLight, list:Vec::new()};
            return_value.push(trans_table);
            current_map = return_value.last_mut().unwrap();
            continue;
        }
        if line.starts_with("light-to-temperature")
        {
            let trans_table = TranslationTable{translation_step: TranslationStep::LightToTemprature, list:Vec::new()};
            return_value.push(trans_table);
            current_map = return_value.last_mut().unwrap();
            continue;
        }
        if line.starts_with("temperature-to-humidity")
        {
            let trans_table = TranslationTable{translation_step: TranslationStep::TempratureToHumidity, list:Vec::new()};
            return_value.push(trans_table);
            current_map = return_value.last_mut().unwrap();
            continue;
        }
        if line.starts_with("humidity-to-location")
        {
            let trans_table = TranslationTable{translation_step: TranslationStep::HumidityToLocation, list:Vec::new()};
            return_value.push(trans_table);
            current_map = return_value.last_mut().unwrap();
            continue;
        }

        let mut trans_range: TranslationRange = TranslationRange{from: 0, to: 0, amount: 0, from_range: 0..0, to_range: 0..0};
        for (index, number_str) in line.split_whitespace().enumerate()
        {
            match index
            {
                0 => {
                    trans_range.to = number_str.parse::<u64>().unwrap();
                },
                1 => {
                    trans_range.from = number_str.parse::<u64>().unwrap();
                },
                2 => {
                    trans_range.amount = number_str.parse::<u64>().unwrap();
                },
                _ => println!("Well... this is a problem, I was expecting a number string"),
            }
        }
        trans_range.calculate_range();
        current_map.list.push(trans_range);
    }
    return_value
}