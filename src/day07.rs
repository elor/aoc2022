use std::fs::read_to_string;

fn main() {
    let input = read_to_string("res/day07.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let tree = Tree::from_str(input);
    tree.maxsize_dirsum_with_doublecounting(100000)
}

fn part2(input: &str) -> usize {
    let tree = Tree::from_str(input);
    tree.size_of_dir_with_closest_size_above_max_size(30000000)
}

#[derive(Debug, PartialEq)]
enum NodeType {
    File,
    Directory,
}

struct Node {
    name: String,
    node_type: NodeType,
    size: usize,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &str, node_type: NodeType, size: usize) -> Node {
        Node {
            name: name.to_string(),
            node_type,
            size,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn total_size(&self) -> usize {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<usize>()
    }

    fn from_str(input: &str) -> Node {
        let mut words = input.trim().split_whitespace();

        let first = words.next();
        let second = words.next();

        // first == "dir" --> directory
        // first == any number --> file
        match (first, second) {
            (Some("dir"), Some(name)) => Node::new(name, NodeType::Directory, 0),
            (Some(size), Some(name)) => Node::new(name, NodeType::File, size.parse().unwrap()),
            _ => panic!("Invalid input"),
        }
    }

    fn get_child_recursive(&mut self, path: &[String]) -> &mut Node {
        if path.is_empty() {
            self
        } else {
            let child = self.children.iter_mut().find(|c| c.name == path[0]);
            match child {
                Some(c) => c.get_child_recursive(&path[1..]),
                None => panic!("Invalid path"),
            }
        }
    }
}

struct Tree {
    root: Node,
    path: Vec<String>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            root: Node::new("/", NodeType::Directory, 0),
            path: Vec::new(),
        }
    }

    fn dir(&mut self) -> &mut Node {
        self.root.get_child_recursive(&self.path)
    }

    fn total_size(&self) -> usize {
        self.root.total_size()
    }

    fn cd(&mut self, path: &str) {
        if path != "/" && path.contains("/") {
            if path.starts_with("/") {
                self.cd("/");
            }

            for p in path.split("/") {
                self.cd(p);
            }
        } else if path == ".." {
            self.path.pop();
        } else if path == "/" {
            self.path.clear();
        } else {
            self.path.push(path.to_string());
        }
    }

    fn ls(&mut self, input: &str) {
        let node = self.dir();
        for line in input.trim().lines() {
            let child = Node::from_str(line);
            node.add_child(child);
        }
    }

    fn command(&mut self, input: &str) {
        let mut lines = input.trim().lines();
        let first_line = lines.next().unwrap();
        let mut words = first_line.trim().split_whitespace();

        let command = words.next();
        let target = words.next();

        match (command, target) {
            (Some("cd"), Some(path)) => self.cd(path),
            (Some("ls"), None) => lines.for_each(|l| self.ls(l)),
            _ => panic!("Invalid input"),
        };
    }

    fn maxsize_dirsum_with_doublecounting(&self, max_size: usize) -> usize {
        let mut sum = 0;
        let mut stack = vec![&self.root];

        while let Some(node) = stack.pop() {
            if node.node_type == NodeType::Directory {
                if node.total_size() <= max_size {
                    sum += node.total_size();
                }
            }

            stack.extend(node.children.iter());
        }

        sum
    }

    fn size_of_dir_with_closest_size_above_max_size(&self, required_free_space: usize) -> usize {
        let total_disk_size = 70000000;
        let max_used_space = total_disk_size - required_free_space;
        let used_space = self.total_size();
        let min_required_deletion = used_space - max_used_space;

        let mut stack = vec![&self.root];
        let mut closest = self.total_size();

        while let Some(node) = stack.pop() {
            if node.node_type == NodeType::Directory {
                if node.total_size() >= min_required_deletion {
                    if node.total_size() < closest {
                        closest = node.total_size();
                    }
                    stack.extend(node.children.iter());
                }
            }
        }

        closest
    }

    fn from_str(input: &str) -> Tree {
        let mut tree = Tree::new();
        for line in input
            .trim()
            .split('$')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            tree.command(line);
        }
        tree.cd("/");
        tree
    }
}

#[cfg(test)]
mod day07 {
    use super::*;

    #[test]
    fn test_part1() {
        let mut dir = Node::new("dir", NodeType::Directory, 0);
        assert_eq!(dir.total_size(), 0);
        let file = Node::new("file", NodeType::File, 10);
        assert_eq!(file.total_size(), 10);
        dir.add_child(file);
        assert_eq!(dir.total_size(), 10);

        let dir = Node::from_str("dir d");
        assert_eq!(dir.name, "d");
        assert_eq!(dir.node_type, NodeType::Directory);

        let file = Node::from_str("29116 f");
        assert_eq!(file.name, "f");
        assert_eq!(file.node_type, NodeType::File);
        assert_eq!(file.size, 29116);

        let mut traversal = Tree::new();
        traversal.command("ls\ndir d\n29116 f");
        assert_eq!(traversal.dir().name, "/");
        assert_eq!(traversal.dir().total_size(), 29116);

        traversal.command("cd d");
        traversal.command("ls\n123 e\n321 f");
        assert_eq!(traversal.dir().total_size(), 123 + 321);
        traversal.command("cd ..");
        assert_eq!(traversal.dir().total_size(), 29116 + 123 + 321);
        traversal.command("cd d");
        traversal.command("cd /");
        assert_eq!(traversal.dir().total_size(), 29116 + 123 + 321);

        assert_eq!(traversal.maxsize_dirsum_with_doublecounting(5), 0);
        assert_eq!(traversal.maxsize_dirsum_with_doublecounting(500), 123 + 321);

        let test_data = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        let traversal = Tree::from_str(test_data);
        assert_eq!(traversal.maxsize_dirsum_with_doublecounting(100000), 95437);
    }

    #[test]
    fn test_part2() {
        let test_data = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        let traversal = Tree::from_str(test_data);
        assert_eq!(
            traversal.size_of_dir_with_closest_size_above_max_size(30000000),
            24933642
        );
    }
}
