//! Path resolution.

use crate::directory::DirectoryTree;

/// Resolve a path to an inode number.
#[derive(Debug)]
pub struct PathResolver<'a> {
    tree: &'a DirectoryTree,
}

impl<'a> PathResolver<'a> {
    pub fn new(tree: &'a DirectoryTree) -> Self {
        Self { tree }
    }

    /// Resolve an absolute path (starts with '/') to an inode number.
    pub fn resolve(&self, path: &str) -> Result<u32, String> {
        let parts = Self::split_path(path);
        if parts.is_empty() {
            return Ok(0); // root
        }
        let mut current_ino = 0u32;
        for part in parts {
            current_ino = self.tree.lookup(current_ino, part)
                .ok_or_else(|| format!("Path component '{}' not found", part))?;
        }
        Ok(current_ino)
    }

    /// Resolve a path relative to a starting inode.
    pub fn resolve_relative(&self, start_ino: u32, path: &str) -> Result<u32, String> {
        let parts = Self::split_path(path);
        if parts.is_empty() {
            return Ok(start_ino);
        }
        let mut current_ino = start_ino;
        for part in parts {
            match part {
                "." => {} // stay
                ".." => {
                    let parent = self.tree.lookup(current_ino, "..")
                        .ok_or("No parent")?;
                    current_ino = parent;
                }
                _ => {
                    current_ino = self.tree.lookup(current_ino, part)
                        .ok_or_else(|| format!("'{}' not found", part))?;
                }
            }
        }
        Ok(current_ino)
    }

    /// Split a path into components.
    pub fn split_path(path: &str) -> Vec<&str> {
        path.split('/')
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Get the basename of a path.
    pub fn basename(path: &str) -> &str {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        parts.last().copied().unwrap_or("")
    }

    /// Get the dirname of a path.
    pub fn dirname(path: &str) -> String {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() <= 1 {
            "/".to_string()
        } else {
            format!("/{}", parts[..parts.len() - 1].join("/"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_tree() -> DirectoryTree {
        let mut tree = DirectoryTree::new();
        tree.add_entry(0, "home", 1).unwrap();
        tree.add_entry(1, "user", 2).unwrap();
        tree.add_entry(2, "docs", 3).unwrap();
        tree.add_entry(2, "file.txt", 4).unwrap();
        tree
    }

    #[test]
    fn resolve_root() {
        let tree = setup_tree();
        let resolver = PathResolver::new(&tree);
        assert_eq!(resolver.resolve("/"), Ok(0));
    }

    #[test]
    fn resolve_deep_path() {
        let tree = setup_tree();
        let resolver = PathResolver::new(&tree);
        assert_eq!(resolver.resolve("/home/user/docs"), Ok(3));
    }

    #[test]
    fn resolve_file() {
        let tree = setup_tree();
        let resolver = PathResolver::new(&tree);
        assert_eq!(resolver.resolve("/home/user/file.txt"), Ok(4));
    }

    #[test]
    fn resolve_nonexistent() {
        let tree = setup_tree();
        let resolver = PathResolver::new(&tree);
        assert!(resolver.resolve("/nope").is_err());
    }

    #[test]
    fn resolve_relative() {
        let tree = setup_tree();
        let resolver = PathResolver::new(&tree);
        assert_eq!(resolver.resolve_relative(2, "docs"), Ok(3));
    }

    #[test]
    fn resolve_parent() {
        let tree = setup_tree();
        let resolver = PathResolver::new(&tree);
        assert_eq!(resolver.resolve_relative(2, ".."), Ok(1));
    }

    #[test]
    fn basename_test() {
        assert_eq!(PathResolver::basename("/home/user/file.txt"), "file.txt");
        assert_eq!(PathResolver::basename("/home"), "home");
    }

    #[test]
    fn dirname_test() {
        assert_eq!(PathResolver::dirname("/home/user/file.txt"), "/home/user");
        assert_eq!(PathResolver::dirname("/home"), "/");
    }

    #[test]
    fn split_path_test() {
        assert_eq!(PathResolver::split_path("/a/b/c"), vec!["a", "b", "c"]);
        assert_eq!(PathResolver::split_path("/"), Vec::<&str>::new());
    }
}
