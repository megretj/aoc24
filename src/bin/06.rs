use std::clone;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    println!("{}", input);
    // make a matrix out of the input
    let mut grid: Vec<Vec<char>>=Vec::new();
    for (row_id,line) in input.lines().enumerate(){
        let mut row = vec![];
        for (col_id,letter) in line.chars().into_iter().enumerate(){
            row.insert(col_id, letter);
        }
        grid.insert(row_id, row);
    }
    // while in the map, go to the next loc and mark it with X
    let map_height = grid.len();
    let map_width = grid[0].len();
    let mut loc:(usize,usize) = (0,0);
    let mut dir:usize = 0;//0=up,1=right,2=down,3=left
    for (i, row) in grid.clone().into_iter().enumerate(){
        for (j, place) in row.into_iter().enumerate(){
            match place {
                '^' => {
                    loc = (i,j);
                    dir = 0;
                    break
                },
                '>' =>{
                    loc = (i,j);
                    dir = 1;
                    break
                },
                'v' =>{
                    loc = (i,j);
                    dir = 2;
                    break
                },
                '<' =>{
                    loc = (i,j);
                    dir = 3;
                    break
                },
                _ =>{
                }
            }
        }
    }
    grid[loc.0][loc.1] = 'X';
    println!("{:?}",loc);
    loop {
        let mut next_loc = (0,0);
        match dir{
            0 =>{
                if loc.0 == 0{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0-1,loc.1);
            },
            1 =>{
                if loc.1 == map_width-1{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0,loc.1+1);
            },
            2 =>{
                if loc.0 == map_height-1{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0+1,loc.1);
            },
            3 =>{
                if loc.1 == 0{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0,loc.1-1);
            },
            _ =>{
                println!("not a valid dir")
            },
        }
        match grid[next_loc.0][next_loc.1]{
            '#' => {
                dir = (dir+1)%4;
            },
            'X' =>{
                println!("pray there are no loops");
                loc = next_loc;
            },
            '.' =>{
                grid[next_loc.0][next_loc.1] = 'X';
                loc = next_loc;
            }
            _ =>{
                println!("Something weird is going on");
                break
            }
        }
        // println!("{:?}",loc);
    }
    // count the number of X in the map
    let mut total:u32 = 0;
    let mut strgrid = String::new();
    for (i, row) in grid.clone().into_iter().enumerate(){
        for (j, place) in row.into_iter().enumerate(){
            strgrid.push(place);
            if place == 'X'{
                total += 1
            }
        }
        strgrid.push('\n');
    }
    println!("{}",strgrid);
    Some(total)
}

pub fn stuck_in_a_loop(ingrid: Vec<Vec<char>>) -> bool{
    let mut grid = ingrid.clone();
    let mut dirgrid: Vec<Vec<Vec<usize>>> = Vec::new();
    let map_height = grid.len();
    let map_width = grid[0].len();
    let mut loc:(usize,usize) = (0,0);
    let mut dir:usize = 0;//0=up,1=right,2=down,3=left
    for (i, row) in grid.clone().into_iter().enumerate(){
        let mut dirrow: Vec<Vec<usize>> = vec![];
        for (j, place) in row.into_iter().enumerate(){
            dirrow.insert(j, vec![]);
            match place {
                '^' => {
                    loc = (i,j);
                    dir = 0;
                },
                '>' =>{
                    loc = (i,j);
                    dir = 1;
                },
                'v' =>{
                    loc = (i,j);
                    dir = 2;
                },
                '<' =>{
                    loc = (i,j);
                    dir = 3;
                },
                _ =>{
                }
            }
        }
        dirgrid.insert(i,dirrow);
    }
    grid[loc.0][loc.1] = 'X';
    println!("{:?}",loc);
    loop {
        let mut next_loc = (0,0);
        dirgrid[loc.0][loc.1].push(dir);
        match dir{
            0 =>{
                if loc.0 == 0{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0-1,loc.1);
            },
            1 =>{
                if loc.1 == map_width-1{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0,loc.1+1);
            },
            2 =>{
                if loc.0 == map_height-1{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0+1,loc.1);
            },
            3 =>{
                if loc.1 == 0{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0,loc.1-1);
            },
            _ =>{
                println!("not a valid dir")
            },
        }
        match grid[next_loc.0][next_loc.1]{
            '#' => {
                dir = (dir+1)%4;
            },
            'X' =>{
                if dirgrid[next_loc.0][next_loc.1].contains(&dir){
                    return true
                }
                loc = next_loc;
            },
            '.' =>{
                grid[next_loc.0][next_loc.1] = 'X';
                loc = next_loc;
            }
            _ =>{
                println!("Something weird is going on");
                break
            }
        }
        // println!("{:?}",loc);
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("{}", input);
    // make a matrix out of the input
    let mut og_grid: Vec<Vec<char>>=Vec::new();
    for (row_id,line) in input.lines().enumerate(){
        let mut row = vec![];
        for (col_id,letter) in line.chars().into_iter().enumerate(){
            row.insert(col_id, letter);
        }
        og_grid.insert(row_id, row);
    }
    let mut grid = og_grid.clone();
    // while in the map, go to the next loc and mark it with X
    let map_height = grid.len();
    let map_width = grid[0].len();
    let mut loc:(usize,usize) = (0,0);
    let mut dir:usize = 0;//0=up,1=right,2=down,3=left
    for (i, row) in grid.clone().into_iter().enumerate(){
        for (j, place) in row.into_iter().enumerate(){
            match place {
                '^' => {
                    loc = (i,j);
                    dir = 0;
                    break
                },
                '>' =>{
                    loc = (i,j);
                    dir = 1;
                    break
                },
                'v' =>{
                    loc = (i,j);
                    dir = 2;
                    break
                },
                '<' =>{
                    loc = (i,j);
                    dir = 3;
                    break
                },
                _ =>{
                }
            }
        }
    }
    let og_loc = loc;
    grid[loc.0][loc.1] = 'X';
    println!("{:?}",loc);
    loop {
        let mut next_loc = (0,0);
        match dir{
            0 =>{
                if loc.0 == 0{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0-1,loc.1);
            },
            1 =>{
                if loc.1 == map_width-1{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0,loc.1+1);
            },
            2 =>{
                if loc.0 == map_height-1{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0+1,loc.1);
            },
            3 =>{
                if loc.1 == 0{
                    grid[loc.0][loc.1] = 'X';
                    break
                }
                next_loc=(loc.0,loc.1-1);
            },
            _ =>{
                println!("not a valid dir")
            },
        }
        match grid[next_loc.0][next_loc.1]{
            '#' => {
                dir = (dir+1)%4;
            },
            'X' =>{
                println!("pray there are no loops");
                loc = next_loc;
            },
            '.' =>{
                grid[next_loc.0][next_loc.1] = 'X';
                loc = next_loc;
            }
            _ =>{
                println!("Something weird is going on");
                break
            }
        }
        // println!("{:?}",loc);
    }
    // count the number of X in the map
    let mut total:u32 = 0;
    let mut strgrid = String::new();
    for (i, row) in grid.clone().into_iter().enumerate(){
        for (j, place) in row.into_iter().enumerate(){
            strgrid.push(place);
            if place == 'X' && (i,j)!=og_loc{
                println!("trying {}{}",i,j);
                let mut new_grid = og_grid.clone();
                new_grid[i][j] = '#'; 
                if stuck_in_a_loop(new_grid) {
                    total += 1
                }
            }
        }
        strgrid.push('\n');
    }
    println!("{}",strgrid);
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
