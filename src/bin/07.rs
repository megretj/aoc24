use regex::Regex;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
   let mut result: u64=0;
   for line in input.lines(){
        let temp:Vec<&str> = line.split(':').collect();
        // println!("{:?}",temp);
        let test_result = temp[0].parse::<u64>().unwrap();
        let equation = temp[1];
        let numbers: Vec<u64> = equation[1..].split(' ').map(|f| f.parse::<u64>().unwrap()).collect();
        for i in 0.. 0 ^ (1 << numbers.len()-1){
            let mut temp_result = numbers[0];
            for (j,number) in numbers[1..].into_iter().enumerate(){
                // println!("{}", i as u16 >> j & 1);
                if (i as u16 >> j & 1) == 0{
                    temp_result += number;
                }else{
                    temp_result *= number;
                }
            }
            if temp_result == test_result{
                // println!("{} is true on {}", i, test_result);
                result += test_result;
                break
            }
        }
    }
    Some(result)
}

fn valid_operation(result:u64, equation: Vec<u64>) -> bool{
    if equation.is_empty(){
        return result == 0;
    }
    // addition
    let term = equation[equation.len()-1];
    let m = 10_u64.pow(term.ilog10() + 1);
    return (result-term > 0 && valid_operation(result-term, equation[..equation.len()-1].to_vec()))|| (result%term == 0 && valid_operation(result/term, equation[..equation.len()-1].to_vec())) || (result%m == term || valid_operation(result / m, equation[..equation.len()-1].to_vec()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64=0;
    for line in input.lines(){
        let temp:Vec<&str> = line.split(':').collect();
        // println!("{:?}",temp);
        let test_result = temp[0].parse::<u64>().unwrap();
        let equation = temp[1];
        let numbers: Vec<u64> = equation[1..].split(' ').map(|f| f.parse::<u64>().unwrap()).collect();
        if valid_operation(test_result, numbers){
            // println!("{} is true on {}", i, test_result);
            result += test_result;
            break
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
