use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum Node {
    Directory(usize),
    File(usize),
}

impl Node {
    pub fn is_directory(&self) -> bool {
        match self {
            Node::Directory(_) => true,
            _ => false,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            &Node::Directory(n) => n,
            &Node::File(n) => n,
        }
    }
}

impl std::ops::AddAssign<usize> for Node {
    fn add_assign(&mut self, rhs: usize) {
        match self {
            Self::Directory(size) => *size += rhs,
            Self::File(size) => *size += rhs,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Hierarchy {
    index: HashMap<String, Node>,
}

impl Hierarchy {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    pub fn space_used(&self) -> usize {
        self.index.get("/").map(Node::size).unwrap_or_default()
    }

    pub fn add_file(&mut self, filepath: String, filesize: usize) {
        self.index.insert(filepath.clone(), Node::File(filesize));

        filepath
            .split('/')
            .scan(String::from(""), |acc, frag| {
                match acc.as_str() {
                    "/" => (),
                    _ => acc.push('/')
                };
                acc.push_str(frag);
                Some(acc.clone())
            })
            .for_each(|path| {
                let entry = self.index.entry(path).or_insert(Node::Directory(0));
                *entry += filesize;
            });
    }
}

impl IntoIterator for Hierarchy {
    type Item = <HashMap<String, Node> as IntoIterator>::Item;

    type IntoIter = <HashMap<String, Node> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}