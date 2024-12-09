use std::{collections::HashMap, default, hash::Hash};

use rayon::iter::Empty;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    // 
    let compressed:Vec<u32> = input.chars().filter_map(|x| x.to_digit(10)).collect();
    // let block_id: u32 = 0;
    let mut total: u64=0;
    let mut reverse_idx = compressed.len()-1;
    let mut carry_size=0;
    let mut carry_idx=0;
    let mut to_fill =0;
    let mut uncompressed: Vec<i64> = Vec::new();
    // for (block_id,block_length) in compressed.clone().into_iter().enumerate(){
    //     if block_id%2==0{
    //         uncompressed.extend(vec![block_id/2;block_length]);
            
    //     }else{
    //         // block_length many free places, fill them with the other side of the vec.
    //         to_fill=block_length;
    //         while to_fill > 0{
    //             if reverse_idx%2 == 0 {
    //                 if carry_size>0{
    //                     uncompressed.extend(vec![carry_idx;a]);
    //                 }
    //                 let a = compressed.pop().unwrap();
    //                 if a <= to_fill{
    //                     uncompressed.extend(vec![reverse_idx/2;a]);
    //                     to_fill -= a;
    //                     reverse_idx-=1;
    //                 }else{
    //                     carry_size = a-to_fill;
    //                     carry_idx = reverse_idx%2;
    //                     uncompressed.extend(vec![reverse_idx/2;to_fill]);
    //                     reverse_idx-=1;
    //                 }
    //             }else{
    //                 compressed.pop();
    //                 reverse_idx-=1;
    //             }
                
    //         }
    //     }
    // }
    // println!("{:?}",compressed);
    for (block_id,block_length ) in compressed.into_iter().enumerate(){
        if block_id%2 == 0{
            uncompressed.extend(vec![(block_id/2) as i64;block_length as usize]);
        }else{
            uncompressed.extend(vec![-1;block_length as usize]);
        }
    }
    // println!("{:?}",uncompressed);
    let mut well_formatted: Vec<u32> = Vec::new();
    for (i,block) in uncompressed.clone().into_iter().enumerate(){
        if i == uncompressed.len(){
            if block == -1{
                uncompressed.pop();
            }
            break
        }
        if block == -1{
            loop{
                match uncompressed.pop(){
                    Some(a)=>{
                        match a {
                            -1 => {},
                            _ =>{well_formatted.push(a as u32);break}
                        }
                    },
                    None => {panic!("Theres an issue with popping")}
                }
            }
        }else{
            well_formatted.push(block as u32);
        }
    }
    // println!("{:?}",well_formatted);
    for i in well_formatted.into_iter().enumerate(){
        total += (i.0 as u64)*(i.1 as u64);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let compressed:Vec<u32> = input.chars().filter_map(|x| x.to_digit(10)).collect();
    // let block_id: u32 = 0;
    let mut total: u64=0;
    let mut reverse_idx = compressed.len()-1;
    let mut carry_size=0;
    let mut carry_idx=0;
    let mut to_fill =0;
    let mut uncompressed: Vec<i64> = Vec::new();
    let mut blocks: HashMap<usize,usize> = HashMap::new();
    let mut emptyblocks: HashMap<usize,usize> = HashMap::new();

    for (block_id,block_length ) in compressed.into_iter().enumerate(){
        if block_id%2 == 0{
            blocks.insert(block_id, block_length as usize);
            uncompressed.extend(vec![(block_id/2) as i64;block_length as usize]);
        }else{
            emptyblocks.insert(block_id,block_length as usize);
            uncompressed.extend(vec![-1;block_length as usize]);
        }
    }
    // println!("{:?}",uncompressed);

    let mut well_formatted: Vec<u32> = Vec::new();

    for (i,block) in uncompressed.clone().into_iter().enumerate(){
        if i == uncompressed.len(){
            if block == -1{
                uncompressed.pop();
            }
            break
        }
        if block == -1{
            loop{
                match uncompressed.pop(){
                    Some(a)=>{
                        match a {
                            -1 => {},
                            _ =>{well_formatted.push(a as u32);break}
                        }
                    },
                    None => {panic!("Theres an issue with popping")}
                }
            }
        }else{
            well_formatted.push(block as u32);
        }
    }
    // println!("{:?}",well_formatted);
    for i in well_formatted.into_iter().enumerate(){
        total += (i.0 as u64)*(i.1 as u64);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
