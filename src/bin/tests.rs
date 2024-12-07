use rayon::prelude::*;

// returns prefix if n ends with suffix
fn ends_with(n: usize, suffix: usize) -> Option<usize> {
    let n_digits = n.ilog10() + 1;
    let suffix_digits = suffix.ilog10() + 1;
    if n_digits < suffix_digits {
        return None;
    }
    if n % 10usize.pow(suffix_digits) == suffix {
        return Some(n / 10usize.pow(suffix_digits));
    }
    None
}

fn test(target: usize, operands: &[usize]) -> bool {
    let mut stack = vec![(operands.len() - 1, target)];

    while let Some((i, target)) = stack.pop() {
        let n = operands[i];
        if i == 0 {
            if n == target {
                return true;
            }
            continue;
        }
        if n > target {
            continue;
        }
        // add
        stack.push((i - 1, target - n));
        // mul
        if target % n == 0 {
            stack.push((i - 1, target / n));
        }
        // concat
        if let Some(prefix) = ends_with(target, n) {
            stack.push((i - 1, prefix));
        }
    }

    false
}

fn main() {
    let input = include_str!("../../data/inputs/07.txt")
        .lines()
        .map(|line| {
            let (target, operands) = line.split_once(": ").unwrap();
            let target = target.parse::<usize>().unwrap();
            let operands = operands
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (target, operands)
        })
        .collect::<Vec<_>>();
    let res = input
        .into_par_iter()
        .filter_map(|(target, operands)| {
            if test(target, &operands) {
                Some(target)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("{}", res);
}
