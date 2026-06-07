//! Inode representation.

/// File type for an inode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    RegularFile,
    Directory,
    SymbolicLink,
}

/// A simulated inode.
#[derive(Debug, Clone)]
pub struct Inode {
    pub ino: u32,
    pub file_type: FileType,
    pub size: u64,
    pub blocks: Vec<u32>,
    pub link_count: u32,
    pub permissions: u16,
}

impl Inode {
    /// Create a new regular file inode.
    pub fn new_file(ino: u32, first_block: u32, size: u64) -> Self {
        Self {
            ino,
            file_type: FileType::RegularFile,
            size,
            blocks: vec![first_block],
            link_count: 1,
            permissions: 0o644,
        }
    }

    /// Create a new directory inode.
    pub fn new_dir(ino: u32, first_block: u32) -> Self {
        Self {
            ino,
            file_type: FileType::Directory,
            size: 0,
            blocks: vec![first_block],
            link_count: 2, // . and ..
            permissions: 0o755,
        }
    }

    /// Create an empty inode.
    pub fn empty(ino: u32) -> Self {
        Self {
            ino,
            file_type: FileType::RegularFile,
            size: 0,
            blocks: vec![],
            link_count: 0,
            permissions: 0o644,
        }
    }

    /// Add a block to this inode.
    pub fn add_block(&mut self, block: u32) {
        if !self.blocks.contains(&block) {
            self.blocks.push(block);
        }
    }

    /// Number of blocks.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Check if this is a directory.
    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }

    /// Check if this is a regular file.
    pub fn is_file(&self) -> bool {
        self.file_type == FileType::RegularFile
    }

    /// Increment link count.
    pub fn link(&mut self) {
        self.link_count += 1;
    }

    /// Decrement link count. Returns true if count reaches zero.
    pub fn unlink(&mut self) -> bool {
        if self.link_count > 0 {
            self.link_count -= 1;
        }
        self.link_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_file_inode() {
        let i = Inode::new_file(1, 10, 512);
        assert_eq!(i.ino, 1);
        assert_eq!(i.file_type, FileType::RegularFile);
        assert_eq!(i.size, 512);
        assert_eq!(i.block_count(), 1);
        assert!(i.is_file());
        assert!(!i.is_dir());
    }

    #[test]
    fn new_dir_inode() {
        let i = Inode::new_dir(2, 20);
        assert_eq!(i.file_type, FileType::Directory);
        assert_eq!(i.link_count, 2);
        assert!(i.is_dir());
    }

    #[test]
    fn add_blocks() {
        let mut i = Inode::empty(3);
        assert_eq!(i.block_count(), 0);
        i.add_block(5);
        i.add_block(6);
        i.add_block(5); // duplicate ignored
        assert_eq!(i.block_count(), 2);
    }

    #[test]
    fn link_unlink_cycle() {
        let mut i = Inode::new_file(4, 10, 100);
        i.link();
        assert_eq!(i.link_count, 2);
        assert!(!i.unlink());
        assert_eq!(i.link_count, 1);
        assert!(i.unlink()); // reaches zero
    }
}
