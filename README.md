# fs-layout

Filesystem layout simulator for research and education.

Simulates the core data structures of a Unix-like filesystem:

- **Inodes** — File metadata (size, type, block pointers)
- **Directory tree** — Hierarchical directory structure
- **Block allocation bitmap** — Free/used block tracking
- **Block management** — Data block operations
- **Path resolution** — Absolute and relative path resolution

## Usage

```rust
use fs_layout::inode::Inode;
use fs_layout::directory::DirectoryTree;
use fs_layout::bitmap::BlockBitmap;
use fs_layout::path::PathResolver;

let mut bitmap = BlockBitmap::new(64);
let mut tree = DirectoryTree::new();

let blk = bitmap.allocate().unwrap();
let inode = Inode::new_file(42, blk, 1024);
tree.add_entry(0, "hello.txt", inode.ino).unwrap();
```

No external dependencies — pure `std`.

## License

MIT
