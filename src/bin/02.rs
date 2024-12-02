advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut n_safe:u32 = 0;
    for line in input.lines(){
        let vline = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        print!("{:?}", vline);
        if line_is_safe(vline){
            n_safe += 1;
            println!(" is safe");
        }else {
            println!(" !!!!!")
        }
    }
    Some(n_safe)
}

pub fn line_is_safe(line:Vec<i32>) -> bool{
    let mut previous_number:i32 = 0;
    let mut increasing:bool = false; 
    for (idx,number) in line.into_iter().enumerate(){
        if idx == 0{
            previous_number = number;
            continue;
        }
        if idx == 1{
            if number > previous_number{
                increasing = true;
            }
        }
        if (number-previous_number).abs() > 3 || (number-previous_number).abs() == 0 || (increasing && (number < previous_number))|| (!increasing && (number > previous_number)){
            return false;
        };
        previous_number = number;
    }
    return true;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut n_safe:u32 = 0;
    for line in input.lines(){
        let vline = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        print!("{:?}", vline);
        if line_is_nearly_safe(vline,false){
            n_safe += 1;
            println!(" is safe");
        }else {
            println!(" !!!!!")
        }
    }
    Some(n_safe)
}

pub fn line_is_nearly_safe(line:Vec<i32>, removal:bool) -> bool{
    let mut previous_number:i32 = 0;
    let mut increasing:bool = false; 
    let mut remove_before =line.clone();
    let mut remove_after =line.clone();
    let mut remove_before_after =line.clone();
    for (idx,number) in line.into_iter().enumerate(){
        if idx == 0{
            previous_number = number;
            continue;
        }
        if idx == 1{
            if number > previous_number{
                increasing = true;
            }
        }
        if (number-previous_number).abs() > 3 || (number-previous_number).abs() == 0 || (increasing && (number < previous_number))|| (!increasing && (number > previous_number)){
            if !removal{
                remove_before.remove(idx);
                remove_after.remove(idx-1);
                if idx > 1{
                    remove_before_after.remove(idx-2);
                    return line_is_nearly_safe(remove_before, true) || line_is_nearly_safe(remove_after, true) || line_is_nearly_safe(remove_before_after, true);
                }
                return line_is_nearly_safe(remove_before, true) || line_is_nearly_safe(remove_after, true);
            }
            return false;
        };
        previous_number = number;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
