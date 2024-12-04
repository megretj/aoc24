advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // process the input into a matrix
    // pad the matrix with 4 rows or cols of .
    // let mut grid = HashMap::new();
    // for i in 1..4{
    //     for j in 1..4{
    //         grid.insert([i,j], '.');
    //     }
    // }
    // for (row,line) in input.lines().enumerate(){
    //     for (col,letter) in line.chars().into_iter().enumerate(){
    //         grid.insert()
    //     }
    //         row.append(line.clone().chars().collect());
    //         row.extend(['.','.','.','.']);
    //         println!("{:?}",row);
    // }

    let mut grid: Vec<Vec<char>>=Vec::new();
    let mut init = true;
    for (row_id,line) in input.lines().enumerate(){
        let mut row = vec!['.','.','.','.','.','.','.','.'];
        for (col_id,letter) in line.chars().into_iter().enumerate(){
            row.insert(col_id+4, letter);
        }
        if init{
            let row_len=row.len();
            let empty_row = vec!['.';row_len];
            for _ in 1..8{
                grid.push(empty_row.clone());
            }
            init = false
        }
        grid.insert(row_id+4, row);
    }

    // println!("{:?}",grid);
    // check every 9x9 square around each character if there are XMAS
    let nrows = grid.len();
    let ncols=grid[0].len();
    let mut total_words:u32 = 0;
    let xmas = vec!['X','M','A','S'];
    for i in 0..nrows{
        for j in 0..ncols{
            if grid[i][j] =='X'{
                let potentials: Vec<Vec<(isize,isize)>> =vec![
                    vec![(0,0),(0,1),(0,2),(0,3)],vec![(0,0),(0,-1),(0,-2),(0,-3)],
                    vec![(0,0),(1,0),(2,0),(3,0)],vec![(0,0),(-1,0),(-2,0),(-3,0)],
                    vec![(0,0),(1,1),(2,2),(3,3)],vec![(0,0),(1,-1),(2,-2),(3,-3)],vec![(0,0),(-1,1),(-2,2),(-3,3)],vec![(0,0),(-1,-1),(-2,-2),(-3,-3)]];
                for potential in potentials.into_iter(){
                    let mut converted =true;
                    for (l,letter ) in xmas.clone().into_iter().enumerate(){
                        if grid[(i as isize +potential[l].0) as usize][(j as isize +potential[l].1) as usize] != letter {
                            converted = false;
                            break;
                        }
                    }
                    if converted{
                        total_words+=1
                    }
                }
            }
        }
    }
    Some(total_words)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>>=Vec::new();
    let mut init = true;
    for (row_id,line) in input.lines().enumerate(){
        let mut row = vec!['.','.','.','.','.','.','.','.'];
        for (col_id,letter) in line.chars().into_iter().enumerate(){
            row.insert(col_id+4, letter);
        }
        if init{
            let row_len=row.len();
            let empty_row = vec!['.';row_len];
            for _ in 1..8{
                grid.push(empty_row.clone());
            }
            init = false
        }
        grid.insert(row_id+4, row);
    }
    let nrows = grid.len();
    let ncols=grid[0].len();
    let mut total_words:u32 = 0;
    let xmas = vec!['M','S','M','S'];
    for i in 0..nrows{
        for j in 0..ncols{
            if grid[i][j] =='A'{
                let potentials: Vec<Vec<(isize,isize)>> =vec![
                    vec![(-1,-1),(1,1),(1,-1),(-1,1)],vec![(-1,-1),(1,1),(-1,1),(1,-1)],
                    vec![(1,1),(-1,-1),(1,-1),(-1,1)],vec![(1,1),(-1,-1),(-1,1),(1,-1)]];
                for potential in potentials.into_iter(){
                    let mut converted =true;
                    for (l,letter ) in xmas.clone().into_iter().enumerate(){
                        if grid[(i as isize +potential[l].0) as usize][(j as isize +potential[l].1) as usize] != letter {
                            converted = false;
                            break;
                        }
                    }
                    if converted{
                        total_words+=1
                    }
                }
            }
        }
    }
    Some(total_words)
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
