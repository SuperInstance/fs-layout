//! Filesystem layout simulation.
//!
//! Provides inodes, directory trees, block allocation bitmaps,
//! block management, and path resolution.

pub mod inode;
pub mod directory;
pub mod bitmap;
pub mod block;
pub mod path;
