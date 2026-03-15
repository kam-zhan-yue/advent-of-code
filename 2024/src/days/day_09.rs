use std::fmt;

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> u128 {
    let disk = Disk::from_string(input).reorganise();
    println!("{}", disk);
    let mut result = 0u128;
    for (i, val) in disk.memory.into_iter().enumerate() {
        if val == -1 {
            continue;
        }
        result += (i as i32 * val) as u128;
    }
    result
}

fn part_two(_input: &str) -> usize {
    0
}


#[derive(Debug, Clone)]
struct Disk {
    memory: Vec<i32>
}

impl Disk {
    pub fn from_string(raw: &str) -> Self {
        let mut memory: Vec<i32> = Vec::new();
        let mut index = 0i32;
        for (i, c) in raw.trim().chars().enumerate() {
            let value: u32 = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                let mut file = vec![index; value.try_into().unwrap()];
                memory.append(&mut file);
                index += 1;
                // File
            } else {
                let mut free = vec![-1i32; value.try_into().unwrap()];
                memory.append(&mut free);
            }
        }

        Disk { memory }
    }

    pub fn reorganise(&self) -> Disk {
        let mut disk = self.clone();
        let mut left = 0usize;
        let mut right = disk.memory.len() - 1;
        while left <= right {
            // Remove all the free space at the end of the disk
            while disk.memory[right] == -1 && right > left {
                disk.memory.pop();
                right -= 1;
            }
            left += 1;
            // If at a free space, we want to swap
            if disk.memory[left] == -1 {
                disk.memory[left] = disk.memory[right];
                disk.memory[right] = -1;
                disk.memory.pop();
            }
        }
        disk
    }
}


impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for i in self.memory.clone() {
            if i == -1 {
                output.push('.');
            } else {
                output.push_str(&i.to_string());
            }
        }
        write!(f, "{}", output)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";

    #[test]
    pub fn test_part_one() {
        assert_eq!(part_one(INPUT), 1928);
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(part_two(INPUT), 0);
    }
}
