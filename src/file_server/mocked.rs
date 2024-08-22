use crate::file_server::{FileNode, FileType};

pub static FILE_SERVER: FileNode = FileNode {
    file_type: FileType::Folder,
    name: "",
    data: None,
    next: Some(&[
        FileNode {
            file_type: FileType::Folder,
            name: "photos",
            data: None,
            next: Some(&[FileNode {
                file_type: FileType::File,
                name: "photo1.png",
                data: Some("TmV2ZXIgZ29ubmEgZ2l2ZSB5b3UgdXA="),
                next: None,
            }]),
        },
        FileNode {
            file_type: FileType::File,
            name: "important.txt",
            data: Some("Important Text"),
            next: None,
        },
        FileNode {
            file_type: FileType::File,
            name: "ping.txt",
            data: Some("Pong"),
            next: None,
        },
    ]),
};

#[cfg(test)]
mod file_server_tests {
    use super::*;

    #[test]
    fn find_important_file() {
        let path = "/important.txt"
            .split_terminator('/')
            .map(|e| e.to_string())
            .collect();
        let data = FILE_SERVER.get(path);

        assert_eq!(data, Some("Important Text".to_string()));
    }

    #[test]
    fn find_photo() {
        let path = "/photos/photo1.png"
            .split_terminator('/')
            .map(|e| e.to_string())
            .collect();

        let data = FILE_SERVER.get(path);

        assert_eq!(data, Some("TmV2ZXIgZ29ubmEgZ2l2ZSB5b3UgdXA=".to_string()));
    }

    #[test]
    fn file_does_not_exist() {
        let path = "/abc/def/doesnt_exist.pdf"
            .split_terminator('/')
            .map(|e| e.to_string())
            .collect();

        let data = FILE_SERVER.get(path);

        assert_eq!(data, None);
    }

    #[test]
    fn folder_exist_but_file_dont() {
        let path = "/photos/photo10000.png"
            .split_terminator('/')
            .map(|e| e.to_string())
            .collect();

        let data = FILE_SERVER.get(path);

        assert_eq!(data, None);
    }
}
