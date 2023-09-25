use std::{collections::HashMap, io::BufRead};

#[derive(Debug)]
enum DirectoryItem {
    File(usize),
    Directory(Directory),
}

#[derive(Debug)]
struct Directory {
    content: std::collections::HashMap<String, DirectoryItem>,
}

impl DirectoryItem {
    pub fn size(&self) -> usize {
        match self {
            DirectoryItem::File(size) => *size,
            DirectoryItem::Directory(directory) => directory.size(),
        }
    }
}

impl Directory {
    pub fn new(
        mut reader: std::io::BufReader<std::fs::File>,
    ) -> ::std::result::Result<Directory, std::io::Error> {
        let mut directory = Directory {
            content: HashMap::new(),
        };

        Self::parse(&mut reader, &mut directory)?;
        Ok(directory)
    }

    pub fn size(&self) -> usize {
        self.content.iter().map(|(_, v)| v.size()).sum()
    }

    fn parse(
        reader: &mut std::io::BufReader<std::fs::File>,
        current_dir: &mut Directory,
    ) -> ::std::result::Result<(), std::io::Error> {
        loop {
            let mut line = String::new();
            if let Ok(n) = reader.read_line(&mut line) {
                if n == 0 {
                    return Ok(());
                }
            }
            let mut split = line.split_ascii_whitespace();
            match split.next() {
                Some("$") => match split.next() {
                    Some("ls") => {}
                    Some("cd") => match split.next() {
                        Some("/") => {}
                        Some("..") => return Ok(()),
                        Some(item) => {
                            if let Some(item) = current_dir.content.get_mut(item) {
                                match item {
                                    DirectoryItem::File(file) => {
                                        println!("{} is a file!", file);
                                    }
                                    DirectoryItem::Directory(dir) => {
                                        Self::parse(reader, dir)?;
                                    }
                                }
                            } else {
                                println!("No such file: {}!", item);
                            }
                        }
                        None => println!("cd - missing argument"),
                    },
                    cmd => {
                        println!("Invalid command: {}", cmd.unwrap_or("*none*"));
                    }
                },
                Some("dir") => {
                    if let Some(key) = split.next() {
                        let dir = DirectoryItem::Directory(Directory {
                            content: HashMap::new(),
                        });
                        current_dir.content.insert(String::from(key), dir);
                    }
                }
                Some(other) => {
                    if let Ok(size) = other.parse::<usize>() {
                        if let Some(key) = split.next() {
                            let file = DirectoryItem::File(size);
                            current_dir.content.insert(String::from(key), file);
                        }
                    }
                }
                None => {
                    println!("Invalid input");
                }
            }
        }
    }
}

fn exercise1_impl(directory: &Directory, path: String, threshold: usize) -> usize {
    let size = directory.size();
    (if size < threshold {
        println!("{path}: {size}");
        size
    } else {
        0
    }) + directory
        .content
        .iter()
        .map(|(name, item)| match item {
            DirectoryItem::Directory(child_dir) => {
                let new_path = path.clone() + "/" + name;
                exercise1_impl(child_dir, new_path, threshold)
            }
            DirectoryItem::File(_) => 0,
        })
        .sum::<usize>()
}

fn exercise2_impl(directory: &Directory, threshold: usize) -> Vec<(String, usize)> {
    let mut result: Vec<(String, usize)> = vec![];
    result.reserve(directory.content.len());
    for (name, item) in directory.content.iter() {
        match item {
            DirectoryItem::Directory(child_dir) => {
                let child_size = child_dir.size();
                if child_size >= threshold {
                    result.push((name.clone(), child_size));
                    let mut child_candidates = exercise2_impl(&child_dir, threshold);
                    result.append(&mut child_candidates);
                }
            }
            DirectoryItem::File(_) => {}
        }
    }
    return result;
}

fn exercise1(filesystem: &Directory, threshold: usize) {
    let result = exercise1_impl(filesystem, String::new(), threshold);
    println!("result (ex1): {result}");
}

fn exercise2(filesystem: &Directory, total: usize, required: usize) {
    let free_space = total - filesystem.size();
    let space_to_free = required.saturating_sub(free_space);

    let mut candidates = exercise2_impl(filesystem, space_to_free);
    candidates.sort_by(|(_, lsize), (_, rsize)| lsize.partial_cmp(rsize).unwrap());
    println!("free_space: {free_space}, space_to_free: {space_to_free}");
    println!("cancidates: {:?}", candidates);
    let result = candidates.get(0).unwrap();
    println!("result: {:?}", result);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("day7/input.txt")?;
    let reader = std::io::BufReader::new(file);
    let filesystem = Directory::new(reader)?;
    exercise1(&filesystem, 100000);
    exercise2(&filesystem, 70000000, 30000000);
    Ok(())
}
