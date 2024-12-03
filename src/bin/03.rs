advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re =Regex::new(r"mul\(((\d{1,3}),(\d{1,3}))\)").unwrap();
    let mut total:u32=0;
    for (_,[_,b,c]) in re.captures_iter(input).map(|x| x.extract()){
        total += b.parse::<u32>().unwrap()*c.parse::<u32>().unwrap();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re =Regex::new(r"mul\((?<multiplication>(?<m1>\d{1,3}),(?<m2>\d{1,3}))\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let mut total:u32=0;
    let mut activated =true;
    for capture in re.captures_iter(input){
        match capture.name("multiplication"){
            Some(_)=>{
                if activated{
                    total += capture.name("m1").unwrap().as_str().parse::<u32>().unwrap()*capture.name("m2").unwrap().as_str().parse::<u32>().unwrap();
                }
            },
            None => {}
        }
        match capture.name("do"){
            Some(_)=> activated=true,
            None => {}
        }
        match capture.name("dont"){
            Some(_)=> activated=false,
            None => {}
        }
    }
    Some(total)
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
