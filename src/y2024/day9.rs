use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Disk Fragmenter";

#[derive(Debug, Clone)]
struct File {
    id: u32,
    space: Space,
}

#[derive(Debug, Clone)]
struct Space {
    size: u32,
    location: u32,
}

pub struct Solution {
    files: Vec<File>,
    free_space: Vec<Space>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let input = input
            .trim()
            .chars()
            .map(|l| l.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        let mut location = 0;
        let mut files: Vec<File> = Vec::with_capacity(input.len() / 2 + 1);
        let mut free_space = Vec::with_capacity(input.len() / 2);

        for chunk in input.chunks(2) {
            match chunk {
                [file_size, free_size] => {
                    files.push(File {
                        id: files.len() as u32,
                        space: Space {
                            size: *file_size,
                            location,
                        },
                    });
                    location += file_size;
                    if free_size > &0 {
                        free_space.push(Space {
                            size: *free_size,
                            location,
                        });
                        location += free_size;
                    }
                }
                [file_size] => {
                    files.push(File {
                        id: files.len() as u32,
                        space: Space {
                            size: *file_size,
                            location,
                        },
                    });
                    location += file_size;
                }
                _ => panic!("Invalid input"),
            }
        }

        Self { files, free_space }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let compacted = compact_files(self.files.clone(), self.free_space.clone().into());
        sum_checksums(&compacted).to_string()
    }

    fn part2(&self) -> String {
        let compacted = compact_whole_files(self.files.clone(), self.free_space.clone());
        sum_checksums(&compacted).to_string()
    }
}

fn compact_files(mut files: Vec<File>, mut free_space: VecDeque<Space>) -> Vec<File> {
    let mut compacted = vec![];

    while let Some(mut free) = free_space.pop_front() {
        let mut file = files.pop().unwrap();

        if free.location > file.space.location {
            files.push(file);
            break; // no space left in front of the file
        }

        if file.space.size <= free.size {
            file.space.location = free.location;
            free.location += file.space.size;
            free.size -= file.space.size;
            compacted.push(file);
            if free.size > 0 {
                free_space.push_front(free);
            }
        } else {
            file.space.size -= free.size;

            let new_file = File {
                id: file.id,
                space: free,
            };

            files.push(file);
            compacted.push(new_file);
        }
    }

    files.extend(compacted);
    files
}

fn compact_whole_files(mut files: Vec<File>, mut free_space: Vec<Space>) -> Vec<File> {
    for file in files.iter_mut().rev() {
        match free_space
            .iter()
            .position(|free| free.size >= file.space.size)
        {
            Some(index) if free_space[index].location > file.space.location => {} // no space left in front of the file
            Some(index) if free_space[index].size == file.space.size => {
                file.space = free_space.remove(index);
            }
            Some(index) => {
                let free = free_space.get_mut(index).unwrap();
                file.space.location = free.location;

                free.location += file.space.size;
                free.size -= file.space.size;
            }
            None => {} // no space left for this file
        }
    }
    files
}

fn sum_checksums(files: &[File]) -> u64 {
    files.iter().map(|f| f.checksum()).sum()
}

impl File {
    fn checksum(&self) -> u64 {
        (self.space.location..self.space.location + self.space.size)
            .map(|location| self.id as u64 * location as u64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "1928");
    }

    #[test]
    fn test_part1_example_no_space() {
        let solution = Solution::with_input(input!(example, 2));
        assert_eq!(solution.part1(), "513");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "6283170117911");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "2858");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "6307653242596");
    }
}
