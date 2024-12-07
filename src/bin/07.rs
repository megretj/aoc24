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

fn valid_operation(result:i64, equation: Vec<i64>,operators:&mut OperationTree,lastop:char) -> bool{
    // println!("{} {:?} {}", result,equation,lastop);
    if equation.is_empty(){
        // println!("{}, equation: {:?}", result, equation);
        return result == 0;
    }
    if result == 0{
        return false
    }
    // addition
    let term = equation[equation.len()-1];
    let m = 10_i64.pow(term.ilog10() + 1);
    let add_child_tree = OperationTree{
        operation: '+',
        children: Vec::new(),
    };
    let mult_child_tree = OperationTree{
        operation: '*',
        children: Vec::new(),
    };
    let concat_child_tree = OperationTree{
        operation: '|',
        children: Vec::new(),
    };
    //operators.children.push(child_tree);
    let place = operators.children.len();
    // println!("term {}, result{}", term, result);
    return ((result-term) >= 0 && valid_operation(result-term, equation[..equation.len()-1].to_vec(), {operators.children.push(add_child_tree); &mut operators.children[place]},'+'))
    || (result%term == 0 && valid_operation(result/term, equation[..equation.len()-1].to_vec(), {operators.children.push(mult_child_tree); &mut operators.children[place]},'*'))
    || (result%m == term && valid_operation(result / m, equation[..equation.len()-1].to_vec(),{operators.children.push(concat_child_tree); &mut operators.children[place]},'|'))
}

#[derive(Debug)]
pub struct OperationTree{
    pub operation: char,
    pub children: Vec<OperationTree>,
}

pub fn print_the_tree(operators:& OperationTree)->String{
    if operators.children.is_empty(){
        return String::from(operators.operation)
    }
    let mut s = String::new();
    s.push(operators.operation);
    s.push_str("[ ");
    for child in operators.children.iter(){
        s.push_str(print_the_tree(& child).as_str());
        s.push_str(",");
    }
    s.pop();
    s.push(']');
    s
}
// impl std::fmt::Display for OperationTree {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         if self.children.is_empty(){
//             return write!(f,"{}", self.operation)
//         }
//         let children: Vec<String> = self.children.into_iter().map(|x| format!("{}",x)).collect();
//         write!(f, "( {}, [{:?}])", self.operation, children)
//     }
// }


pub fn part_two(input: &str) -> Option<i64> {
    let mut result: i64=0;
    for line in input.lines(){
        let temp:Vec<&str> = line.split(':').collect();
        // println!("{:?}",temp);
        let test_result = temp[0].parse::<i64>().unwrap();
        let equation = temp[1];
        let numbers: Vec<i64> = equation[1..].split(' ').map(|f| f.parse::<i64>().unwrap()).collect();
        let mut op_tree = OperationTree{
            operation: '_',
            children: Vec::new(),
        };
        if valid_operation(test_result, numbers, &mut op_tree,'_'){
            // println!("{}: {}", test_result, print_the_tree(& op_tree));
            result += test_result;
        }
        // if validate(test_result, &numbers,true){
        //     result += test_result;
        //     println!("{}",result);
        //     if test_result == 729{
        //         println!("CAREFUL: {:?}",temp);
        //     }
        // }
    }
    Some(result)
}

fn validate(curr: i64, ns: &[i64], p2: bool) -> bool {
    if ns.is_empty() {
        return curr == 0;
    }
    if curr < 0 {
        return false;
    }
    let n = ns[ns.len() - 1];
    let m = 10_i64.pow(n.ilog10() + 1);
    (p2 && curr % m == n && validate(curr / m, &ns[..ns.len() - 1], p2))
        || (curr % n == 0 && validate(curr / n, &ns[..ns.len() - 1], p2))
        || validate(curr - n, &ns[..ns.len() - 1], p2)
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
        assert_eq!(result, Some(11387));
    }
}
