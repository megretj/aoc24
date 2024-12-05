advent_of_code::solution!(5);
use regex::Regex;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let rules_and_updates:Vec<&str> = input.split("\n\n").collect();
    // println!("{:?}", rules_and_updates[0]);
    let mut rules_map: HashMap<u32,Vec<u32>> = HashMap::new();
    for rule in rules_and_updates[0].lines(){
        let before_after: Vec<&str> = rule.split("|").collect();
        let before = before_after[0].parse::<u32>().unwrap();
        let after = before_after[1].parse::<u32>().unwrap();
        let rule_map = rules_map.entry(before).or_insert(vec![]);
        rule_map.push(after);
    }
    let mut middle_counter: u32 =0;
    for update in rules_and_updates[1].lines(){
        if is_valid_update(update, &rules_map){
            middle_counter += middle_number(update);
        }
    }
    Some(middle_counter)
}

pub fn is_valid_update(update:&str, rules:&HashMap<u32,Vec<u32>>) -> bool{
    let pages: Vec<u32> = update.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    for (at,before) in pages.clone().into_iter().enumerate(){
        let followers = rules.get(&before).unwrap();
        for after in pages.split_at(at+1).1{
            if !followers.contains(after){
                return false
            }
        }
    }
    true
}

pub fn middle_number(update:&str) -> u32{
    let pages: Vec<u32> = update.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    return *pages.get((pages.len()-1)/2).unwrap()
}

pub fn recover_update(pages:Vec<u32>,rules:&HashMap<u32,Vec<u32>>)->Vec<u32>{
    for (at,before) in pages.clone().into_iter().enumerate(){
        let followers = rules.get(&before).unwrap();
        let (s1,s2) = pages.split_at(at+1);
        for (ad, after) in s2.into_iter().enumerate(){
            if !followers.contains(after){
                // switch both elements and
                // return the recovered update if not valid
                let mut newpages = pages.clone();
                newpages.swap(at, at+ad+1);
                return recover_update(newpages, rules)
            }
        }
    }
    pages
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules_and_updates:Vec<&str> = input.split("\n\n").collect();
    // println!("{:?}", rules_and_updates[0]);
    let mut rules_map: HashMap<u32,Vec<u32>> = HashMap::new();
    for rule in rules_and_updates[0].lines(){
        let before_after: Vec<&str> = rule.split("|").collect();
        let before = before_after[0].parse::<u32>().unwrap();
        let after = before_after[1].parse::<u32>().unwrap();
        let rule_map = rules_map.entry(before).or_insert(vec![]);
        rule_map.push(after);
    }
    let mut middle_counter: u32 =0;
    for update in rules_and_updates[1].lines(){
        if !is_valid_update(update, &rules_map){
            let pages: Vec<u32> = update.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
            let fixed: Vec<u32> = recover_update(pages,&rules_map);
            middle_counter += fixed.get((fixed.len()-1)/2).unwrap()
        }
    }
    Some(middle_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
