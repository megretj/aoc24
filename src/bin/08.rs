use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug,Clone,PartialEq,Hash,Eq)]
struct loc{
    row: isize,
    col: isize,
}
impl std::fmt::Display for loc {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.row,self.col)
        }
    }

fn check_in_grid(tentative: &loc, max_row:isize, max_col:isize)->bool{
    return tentative.row>=0 && tentative.row<max_row && tentative.col >= 0 && tentative.col<max_col
}
fn resonant_frenquency(a:&loc,b:&loc,n:isize)->loc{
    loc{
        row: a.row + n*(a.row - b.row),
        col: a.col + n*(a.col - b.col)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>>=Vec::new();
    let mut interferences:HashSet<loc> = HashSet::new();
    let mut antennas: HashMap<char, Vec<loc>> = HashMap::new();
    for (row_id,line) in input.lines().enumerate(){
        let mut row = vec![];
        for (col_id,letter) in line.chars().into_iter().enumerate(){
            if letter != '.'{
                let new_loc = loc{
                    row:row_id as isize,
                    col:col_id as isize,
                };
                if antennas.contains_key(&letter){
                    antennas.get_mut(&letter).unwrap().push(new_loc)
                }else{
                    antennas.insert(letter, vec![new_loc]);
                }
            }
            row.insert(col_id, letter);
        }
        grid.insert(row_id, row);
    }
    // println!("{:?}",grid);
    let nrows = grid.len() as isize;
    let ncols= grid[0].len() as isize;
    
    for (antenna_name, antenna_locs) in antennas.into_iter(){
        for (i, first_antenna) in antenna_locs.clone().into_iter().enumerate(){
            for (_,second_antenna) in antenna_locs[i+1..].into_iter().enumerate(){
                let d0=resonant_frenquency(&first_antenna, &second_antenna, 1);
                if check_in_grid(&d0, nrows, ncols){
                    println!("adding loc {} from {} antennas {} and {}", d0, antenna_name, first_antenna, second_antenna);
                    interferences.insert(d0);
                }
                let d1=resonant_frenquency(&second_antenna, &first_antenna, 1);
                if check_in_grid(&d1, nrows, ncols){
                    println!("adding loc {} from {} antennas {} and {}", d1, antenna_name, first_antenna, second_antenna);
                    interferences.insert(d1);
                }
            }
        }
    }
    Some(interferences.len() as u32)

}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>>=Vec::new();
    let mut interferences:HashSet<loc> = HashSet::new();
    let mut antennas: HashMap<char, Vec<loc>> = HashMap::new();
    for (row_id,line) in input.lines().enumerate(){
        let mut row = vec![];
        for (col_id,letter) in line.chars().into_iter().enumerate(){
            if letter != '.'{
                let new_loc = loc{
                    row:row_id as isize,
                    col:col_id as isize,
                };
                if antennas.contains_key(&letter){
                    antennas.get_mut(&letter).unwrap().push(new_loc)
                }else{
                    antennas.insert(letter, vec![new_loc]);
                }
            }
            row.insert(col_id, letter);
        }
        grid.insert(row_id, row);
    }
    // println!("{:?}",grid);
    let nrows = grid.len() as isize;
    let ncols= grid[0].len() as isize;
    
    for (antenna_name, antenna_locs) in antennas.into_iter(){
        for (i, first_antenna) in antenna_locs.clone().into_iter().enumerate(){
            for (_,second_antenna) in antenna_locs[i+1..].into_iter().enumerate(){
                let mut n: isize = 0;
                loop{
                    let d0= resonant_frenquency(&first_antenna, &second_antenna, n);
                    if check_in_grid(&d0, nrows, ncols){
                        // println!("adding loc {} from {} antennas {} and {}", d0, antenna_name, first_antenna, second_antenna);
                        interferences.insert(d0);
                        n += 1;
                    }else{
                        break
                    }
                }
                n = 0;
                loop {
                    let d1= resonant_frenquency(&second_antenna, &first_antenna, n);
                    if check_in_grid(&d1, nrows, ncols){
                        // println!("adding loc {} from {} antennas {} and {}", d1, antenna_name, first_antenna, second_antenna);
                        interferences.insert(d1);
                        n+=1;
                    }else{
                        break
                    }
                }
                
            }
        }
    }
    Some(interferences.len() as u32)
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
        assert_eq!(result, Some(34));
    }
}
