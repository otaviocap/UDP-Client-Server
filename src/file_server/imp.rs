#[derive(Eq, PartialEq, Copy, Clone)]
pub enum FileType {
    Folder,
    File,
}

#[derive(Clone)]
pub struct FileNode<'a> {
    pub file_type: FileType,
    pub name: &'a str,
    pub data: Option<&'a str>,
    pub next: Option<&'a [FileNode<'a>]>,
}

impl<'a> FileNode<'a> {
    pub fn get(&self, path: Vec<String>) -> Option<String> {
        if path.is_empty() || path[0] != self.name {
            return None;
        }

        if path.len() == 1 {
            return match self.file_type {
                FileType::Folder => None,
                FileType::File => self.data.map(|e| e.to_string()),
            };
        }

        match self.file_type {
            FileType::Folder => {
                if let Some(next_nodes) = &self.next {
                    let next_name = &path[1];

                    for node in *next_nodes {
                        if node.name == next_name.as_str() {
                            return node.get(path[1..].to_vec());
                        }
                    }
                }
                None
            }
            FileType::File => None,
        }
    }
}
