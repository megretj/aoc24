advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    for line in input.lines(){
        let res:Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        col1.push(res[0]);
        col2.push(res[1]);
    }
    col1.sort();
    col2.sort();

    let mut total_diff:u32 = 0;
    for (id,row) in col1.iter().enumerate(){
        total_diff += (row-col2[id]).abs() as u32
    }
    return Some(total_diff);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();
    for line in input.lines(){
        let res:Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        col1.push(res[0]);
        col2.push(res[1]);
    }
    let mut total_simscore:u32 =0;
    for row in col1.iter(){
        let mut occurences: u32 = 0;
        for r in col2.iter(){
            if r == row{
                occurences += 1;
            }
        }
        total_simscore += row*occurences;
    }
    return Some(total_simscore);
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
