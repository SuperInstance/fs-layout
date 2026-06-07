//! Directory tree simulation.

/// A directory entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    pub name: String,
    pub ino: u32,
}

/// A simulated directory tree.
///
/// Maps parent inode → children entries.
#[derive(Debug)]
pub struct DirectoryTree {
    /// parent_ino → Vec<DirEntry>
    entries: std::collections::HashMap<u32, Vec<DirEntry>>,
}

impl DirectoryTree {
    pub fn new() -> Self {
        let mut entries = std::collections::HashMap::new();
        // Root directory (ino=0) has . and ..
        entries.insert(0, vec![
            DirEntry { name: ".".into(), ino: 0 },
            DirEntry { name: "..".into(), ino: 0 },
        ]);
        Self { entries }
    }

    /// Add an entry to a parent directory.
    pub fn add_entry(&mut self, parent_ino: u32, name: &str, child_ino: u32) -> Result<(), String> {
        if name.is_empty() || name == "." || name == ".." {
            return Err(format!("Invalid name: '{}'", name));
        }
        let children = self.entries.entry(parent_ino).or_default();
        if children.iter().any(|e| e.name == name) {
            return Err(format!("Entry '{}' already exists", name));
        }
        children.push(DirEntry {
            name: name.to_string(),
            ino: child_ino,
        });
        // Initialize child's directory if it's a directory
        self.entries.entry(child_ino).or_insert_with(|| vec![
            DirEntry { name: ".".into(), ino: child_ino },
            DirEntry { name: "..".into(), ino: parent_ino },
        ]);
        Ok(())
    }

    /// Remove an entry from a directory.
    pub fn remove_entry(&mut self, parent_ino: u32, name: &str) -> Result<u32, String> {
        let children = self.entries.get_mut(&parent_ino).ok_or("Parent not found")?;
        let idx = children.iter().position(|e| e.name == name).ok_or("Entry not found")?;
        let removed = children.remove(idx);
        Ok(removed.ino)
    }

    /// Look up an entry by name in a directory.
    pub fn lookup(&self, parent_ino: u32, name: &str) -> Option<u32> {
        self.entries.get(&parent_ino)?.iter().find(|e| e.name == name).map(|e| e.ino)
    }

    /// List entries in a directory.
    pub fn list(&self, parent_ino: u32) -> Vec<&DirEntry> {
        self.entries.get(&parent_ino).map(|v| v.iter().collect()).unwrap_or_default()
    }

    /// Check if a directory is empty (only . and ..).
    pub fn is_empty_dir(&self, ino: u32) -> bool {
        self.entries.get(&ino).map(|v| v.len() <= 2).unwrap_or(true)
    }
}

impl Default for DirectoryTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_exists() {
        let tree = DirectoryTree::new();
        assert_eq!(tree.list(0).len(), 2); // . and ..
    }

    #[test]
    fn add_entry() {
        let mut tree = DirectoryTree::new();
        tree.add_entry(0, "foo", 1).unwrap();
        assert_eq!(tree.lookup(0, "foo"), Some(1));
    }

    #[test]
    fn duplicate_entry_fails() {
        let mut tree = DirectoryTree::new();
        tree.add_entry(0, "foo", 1).unwrap();
        assert!(tree.add_entry(0, "foo", 2).is_err());
    }

    #[test]
    fn remove_entry() {
        let mut tree = DirectoryTree::new();
        tree.add_entry(0, "foo", 1).unwrap();
        let ino = tree.remove_entry(0, "foo").unwrap();
        assert_eq!(ino, 1);
        assert_eq!(tree.lookup(0, "foo"), None);
    }

    #[test]
    fn nested_directories() {
        let mut tree = DirectoryTree::new();
        tree.add_entry(0, "dir1", 1).unwrap();
        tree.add_entry(1, "file1", 2).unwrap();
        assert_eq!(tree.lookup(1, "file1"), Some(2));
    }

    #[test]
    fn invalid_names() {
        let mut tree = DirectoryTree::new();
        assert!(tree.add_entry(0, "", 1).is_err());
        assert!(tree.add_entry(0, ".", 1).is_err());
        assert!(tree.add_entry(0, "..", 1).is_err());
    }

    #[test]
    fn is_empty_dir() {
        let mut tree = DirectoryTree::new();
        tree.add_entry(0, "dir1", 1).unwrap();
        assert!(tree.is_empty_dir(1)); // only . and ..
        tree.add_entry(1, "file1", 2).unwrap();
        assert!(!tree.is_empty_dir(1));
    }
}
