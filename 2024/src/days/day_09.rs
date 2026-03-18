use std::fmt;

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> u128 {
    Disk::from_string(input).reorganise().calculate_checksum()
}

fn part_two(input: &str) -> u128 {
    Disk::from_file_system(FileSystem::from_string(input).reorganise()).calculate_checksum()
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
        while left < right {
            // If at a free space, we want to swap
            if disk.memory[left] == -1 {
                // We want to get the next free space
                while left < right && disk.memory[right] == -1 {
                    right -= 1;
                }
                disk.memory[left] = disk.memory[right];
                disk.memory[right] = -1;
            }
            left += 1;
        }
        disk
    }

    pub fn calculate_checksum(&self) -> u128 {
        let mut result = 0u128;
        for (i, val) in self.memory.clone().into_iter().enumerate() {
            if val != -1 {
                result += (i as i32 * val) as u128;
            }
        }
        result
    }

    pub fn from_file_system(fs: FileSystem) -> Self {
        let mut memory: Vec<i32> = vec![-1; fs.size];
        for file in fs.files {
            memory.splice(file.index..file.index+file.size, vec![file.value; file.size]);
        }
        Disk { memory }
    }
}

#[derive(Debug, Clone)]
struct FileSystem {
    spaces: Vec<Space>,
    files: Vec<File>,
    size: usize,
}

#[derive(Debug, Clone)]
struct Space {
    index: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct File {
    index: usize,
    size: usize,
    value: i32,
}

impl FileSystem {
    pub fn from_string(raw: &str) -> Self {
        let mut spaces: Vec<Space> = Vec::new();
        let mut files: Vec<File> = Vec::new();
        let mut index = 0usize;
        let mut file_index = 0i32;
        for (i, c) in raw.trim().chars().enumerate() {
            let size: usize = c.to_digit(10).unwrap().try_into().unwrap();
            if i % 2 == 0 {
                files.push(File { index, size, value: file_index,});
                file_index += 1;
                index += size;
            } else {
                spaces.push(Space { index, size });
                index += size;
            }
        }

        FileSystem { spaces, files, size: index }
    }

    pub fn reorganise(&self) -> Self {
        let mut fs = self.clone();
        // Go through all the files from right to left
        for i in (0..fs.files.len()).rev() {
            // Find a space that fits form left to right
            for j in 0..fs.spaces.len() {
                if fs.files[i].index < fs.spaces[j].index {
                    // There is no space that fits
                    break
                } else if fs.files[i].size <= fs.spaces[j].size {
                    // There is a space that fits, so reduce that space!
                    fs.files[i].index = fs.spaces[j].index;
                    fs.spaces[j].index += fs.files[i].size;
                    fs.spaces[j].size += fs.files[i].size;
                    break
                }
            }
        }
        fs
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
        assert_eq!(part_two(INPUT), 2858);
    }
}
