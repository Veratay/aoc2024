use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(mut input: &str) -> Option<u32> {
    input = INPUT;
    println!("{}",input);

    let a = Regex::new(r"(?m)^(\d+)\s+(\d+)").unwrap();
    let b = a.captures_iter(&INPUT).map(|it| it.extract());
    
    let mut v1:Vec<i32> = vec![];
    let mut v2:Vec<i32> = vec![];
    for (_,[x1,x2]) in b {
        println!("{x1} {x2}");
        v1.push(x1.parse().unwrap());
        v2.push(x2.parse().unwrap());
    }

    v1.sort();
    v2.sort();

    let o2 = v2.clone().into_iter().counts();

    let mut s = 0;
    for x in v1.into_iter() {
        s += x*(o2.get(&x).map(|it| *it).unwrap_or(0) as i32);
    }

    
    return Some(s.try_into().unwrap());
    

    // let left = v1.into_iter().zip(v2.into_iter());
    
    // let mut res = 0;
    // for (a,b) in left {
        // res += a.abs_diff(b);
        // println!("{res}");
    // }
    // return Some(res);
    // for x in left {
        // print!("{:?}",x);
    // }
    // return Some(0);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{:?}",result.clone());
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
