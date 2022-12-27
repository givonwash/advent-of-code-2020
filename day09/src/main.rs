use std::{
    cmp::Ordering,
    collections::HashSet,
    io::{self, Read},
};

fn find_xy(nums: &[u64], target: u64) -> Option<(u64, u64)> {
    let mut complements = HashSet::new();

    nums.iter().filter(|n| **n < target).find_map(|n| {
        if let Some(c) = complements.get(&(target - n)) {
            if n != c {
                Some((*n, *c))
            } else {
                None
            }
        } else {
            complements.insert(*n);
            None
        }
    })
}

fn find_weakness(nums: &[u64], start: usize, end: usize, target: u64) -> Option<u64> {
    let (min, max, sum) = &nums[start..end].iter().fold(
        (None, None, 0),
        |(mut amin, mut amax, mut asum): (Option<u64>, Option<u64>, u64), n| {
            amin = amin.map(|mn| mn.min(*n)).or(Some(*n));
            amax = amax.map(|mx| mx.max(*n)).or(Some(*n));
            asum += *n;
            (amin, amax, asum)
        },
    );

    match sum.cmp(&target) {
        Ordering::Greater => find_weakness(nums, start + 1, end, target),
        Ordering::Equal => Some(min.unwrap() + max.unwrap()),
        Ordering::Less => {
            if end < nums.len() - 1 {
                find_weakness(nums, start, end + 1, target)
            } else {
                None
            }
        }
    }
}

fn part_one(nums: &[u64]) {
    let first_fail = nums
        .windows(26)
        .find_map(|win| {
            let (win, num) = win.split_at(25);
            let num = num.first().unwrap();
            match find_xy(win, *num) {
                Some(..) => None,
                None => Some(num),
            }
        })
        .expect("No invalid numbers in input");

    println!("Part One: {first_fail}");
}

fn part_two(nums: &[u64]) {
    let (first_fail_at, first_fail) = nums
        .windows(26)
        .enumerate()
        .find_map(|(i, win)| {
            let (win, num) = win.split_at(25);
            let num = num.first().unwrap();
            match find_xy(win, *num) {
                Some(..) => None,
                None => Some((i, num)),
            }
        })
        .expect("No invalid numbers in input");

    let answer = find_weakness(&nums[..first_fail_at], 0, 1, *first_fail);

    println!("Part Two: {answer:?}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let nums = input
        .lines()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    part_one(&nums);
    part_two(&nums);

    Ok(())
}
