use std::{cell::RefCell, rc::Rc};

use super::DaySolver;

pub struct Solver;

#[derive(Debug, Clone)]
enum DirectoryItem<'a> {
    File { name: &'a str, size: usize },
    Directory(Rc<RefCell<Directory<'a>>>),
}

#[derive(Debug, Clone)]
pub struct Directory<'a> {
    name: &'a str,
    items: Vec<DirectoryItem<'a>>,
    parent: Option<Rc<RefCell<Directory<'a>>>>,
}

impl Directory<'_> {
    fn compute_sizes(&self, res: &mut Vec<usize>) -> usize {
        let size = self
            .items
            .iter()
            .map(|item| match item {
                DirectoryItem::File { name: _, size } => *size,
                DirectoryItem::Directory(dir) => dir.borrow().compute_sizes(res),
            })
            .sum();
        res.push(size);
        size
    }
}

#[derive(Debug)]
enum DirCommand<'a> {
    Cd(&'a str),
    Ls,
}

impl<'a> DirCommand<'a> {
    fn new(s: &'a str) -> Self {
        match s {
            s if s.starts_with("$ cd") => Self::Cd(s.strip_prefix("$ cd ").unwrap()),
            s if s.starts_with("$ ls") => Self::Ls,
            _ => panic!("Invalid command"),
        }
    }
}

impl Solver {
    fn make_root_directory(input_txt: &str) -> Rc<RefCell<Directory<'_>>> {
        let root_dir = Rc::new(RefCell::new(Directory {
            name: "/",
            items: Vec::new(),
            parent: None,
        }));

        // we get exactly one mutable reference, that is the 'current' directory
        let mut curr_dir = root_dir.clone();
        let mut lines = input_txt.lines().peekable();
        loop {
            let cmd_line = lines.next().unwrap();
            let cmd = DirCommand::new(cmd_line);
            match &cmd {
                DirCommand::Cd(dir_name) => {
                    if *dir_name == "/" {
                        curr_dir = root_dir.clone();
                    } else if *dir_name == ".." {
                        // let just go back one directory and resume
                        let parent = {
                            let dir = curr_dir.borrow();
                            dir.parent.clone()
                        };
                        if let Some(parent) = parent {
                            curr_dir = parent.clone();
                        }
                    } else {
                        let item = {
                            let dir = curr_dir.borrow();
                            dir.items.iter().find(|item| matches!(item, DirectoryItem::Directory(dir) if dir.borrow().name == *dir_name)).unwrap().clone()
                        };
                        if let DirectoryItem::Directory(inner) = item {
                            curr_dir = inner.clone();
                        }
                    }
                }
                DirCommand::Ls => {
                    // we parse all the next lines till we get to the next command
                    while let Some(line) = lines.next() {
                        if line.starts_with("dir") {
                            let dir_name = line.strip_prefix("dir ").unwrap();
                            // lets create it if it doesn't exists
                            let dir = {
                                let dir = curr_dir.borrow();
                                dir.items.iter().find(|item| matches!(item, DirectoryItem::Directory(dir) if dir.borrow().name == dir_name)).cloned()
                            };
                            if dir.is_none() {
                                curr_dir
                                    .borrow_mut()
                                    .items
                                    .push(DirectoryItem::Directory(Rc::new(RefCell::new(
                                        Directory {
                                            name: dir_name,
                                            items: Vec::new(),
                                            parent: Some(curr_dir.clone()),
                                        },
                                    ))))
                            }
                        } else {
                            let mut split = line.split_whitespace();
                            let size = split.next().unwrap().parse::<usize>().unwrap();
                            let file_name = split.next().unwrap();
                            let file = {
                                let dir = curr_dir.borrow();
                                dir.items.iter().find(|item| matches!(item, DirectoryItem::File { name, ..} if *name == file_name)).cloned()
                            };
                            if file.is_none() {
                                curr_dir.borrow_mut().items.push(DirectoryItem::File {
                                    name: file_name,
                                    size,
                                })
                            }
                        }

                        if lines.peek().is_none() || lines.peek().unwrap().starts_with('$') {
                            break;
                        }
                    }
                }
            }
            if lines.peek().is_none() {
                break;
            }
        }
        root_dir
    }
}

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let root_dir = Self::make_root_directory(input_txt);
        let mut dir_sizes = Vec::new();
        root_dir.borrow().compute_sizes(&mut dir_sizes);
        let total_size = dir_sizes.iter().max().unwrap();
        let space_remaining = 70000000 - total_size;
        let spaced_needed = 30000000 - space_remaining;
        dir_sizes
            .iter()
            .filter(|size| **size >= spaced_needed)
            .min()
            .unwrap()
            .to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        let root_dir = Self::make_root_directory(input_txt);
        let mut dir_sizes = Vec::new();
        root_dir.borrow().compute_sizes(&mut dir_sizes);
        dir_sizes
            .iter()
            .filter(|size| **size <= 100000)
            .sum::<usize>()
            .to_string()
    }
}
