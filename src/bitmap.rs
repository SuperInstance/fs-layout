//! Block allocation bitmap.

/// A bitmap for tracking allocated/free data blocks.
#[derive(Debug, Clone)]
pub struct BlockBitmap {
    bits: Vec<bool>,
    total: usize,
}

impl BlockBitmap {
    /// Create a new bitmap with `total` blocks, all free.
    pub fn new(total: usize) -> Self {
        Self {
            bits: vec![false; total],
            total,
        }
    }

    /// Allocate the first free block. Returns its index.
    pub fn allocate(&mut self) -> Option<usize> {
        for (i, allocated) in self.bits.iter_mut().enumerate() {
            if !*allocated {
                *allocated = true;
                return Some(i);
            }
        }
        None
    }

    /// Allocate a specific block.
    pub fn allocate_specific(&mut self, idx: usize) -> bool {
        if idx < self.total && !self.bits[idx] {
            self.bits[idx] = true;
            true
        } else {
            false
        }
    }

    /// Free a block.
    pub fn free(&mut self, idx: usize) -> bool {
        if idx < self.total && self.bits[idx] {
            self.bits[idx] = false;
            true
        } else {
            false
        }
    }

    /// Check if a block is allocated.
    pub fn is_allocated(&self, idx: usize) -> bool {
        self.bits.get(idx).copied().unwrap_or(false)
    }

    /// Number of free blocks.
    pub fn free_count(&self) -> usize {
        self.bits.iter().filter(|&&b| !b).count()
    }

    /// Number of used blocks.
    pub fn used_count(&self) -> usize {
        self.total - self.free_count()
    }

    /// Total blocks.
    pub fn total(&self) -> usize {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_all_free() {
        let bm = BlockBitmap::new(10);
        assert_eq!(bm.free_count(), 10);
        assert_eq!(bm.used_count(), 0);
    }

    #[test]
    fn allocate_sequential() {
        let mut bm = BlockBitmap::new(3);
        assert_eq!(bm.allocate(), Some(0));
        assert_eq!(bm.allocate(), Some(1));
        assert_eq!(bm.allocate(), Some(2));
        assert_eq!(bm.allocate(), None); // full
    }

    #[test]
    fn free_and_reuse() {
        let mut bm = BlockBitmap::new(2);
        bm.allocate();
        bm.allocate();
        bm.free(0);
        assert_eq!(bm.allocate(), Some(0));
    }

    #[test]
    fn allocate_specific() {
        let mut bm = BlockBitmap::new(5);
        assert!(bm.allocate_specific(3));
        assert!(bm.is_allocated(3));
        assert!(!bm.is_allocated(0));
    }

    #[test]
    fn double_allocate_fails() {
        let mut bm = BlockBitmap::new(5);
        bm.allocate_specific(2);
        assert!(!bm.allocate_specific(2));
    }

    #[test]
    fn free_unallocated_fails() {
        let mut bm = BlockBitmap::new(5);
        assert!(!bm.free(0)); // not allocated
    }

    #[test]
    fn out_of_bounds() {
        let bm = BlockBitmap::new(5);
        assert!(!bm.is_allocated(10));
    }
}
