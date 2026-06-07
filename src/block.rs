//! Data block management.

/// Block size in bytes.
pub const BLOCK_SIZE: u64 = 4096;

/// A simulated data block.
#[derive(Debug, Clone)]
pub struct Block {
    pub index: usize,
    pub data: Vec<u8>,
}

impl Block {
    /// Create a new zeroed block.
    pub fn new(index: usize) -> Self {
        Self {
            index,
            data: vec![0u8; BLOCK_SIZE as usize],
        }
    }

    /// Create a block with specific data (padded/truncated to BLOCK_SIZE).
    pub fn with_data(index: usize, data: &[u8]) -> Self {
        let mut block_data = vec![0u8; BLOCK_SIZE as usize];
        let len = data.len().min(BLOCK_SIZE as usize);
        block_data[..len].copy_from_slice(&data[..len]);
        Self {
            index,
            data: block_data,
        }
    }

    /// Write data at offset.
    pub fn write_at(&mut self, offset: usize, data: &[u8]) -> Result<usize, String> {
        if offset >= BLOCK_SIZE as usize {
            return Err("Offset out of bounds".into());
        }
        let available = (BLOCK_SIZE as usize) - offset;
        let len = data.len().min(available);
        self.data[offset..offset + len].copy_from_slice(&data[..len]);
        Ok(len)
    }

    /// Read data from offset.
    pub fn read_at(&self, offset: usize, len: usize) -> Vec<u8> {
        let end = (offset + len).min(BLOCK_SIZE as usize);
        if offset >= end {
            vec![]
        } else {
            self.data[offset..end].to_vec()
        }
    }

    /// Block index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// How many non-zero bytes.
    pub fn used_bytes(&self) -> usize {
        self.data.iter().filter(|&&b| b != 0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_block_zeroed() {
        let b = Block::new(0);
        assert_eq!(b.data.len(), BLOCK_SIZE as usize);
        assert!(b.data.iter().all(|&x| x == 0));
    }

    #[test]
    fn with_data() {
        let b = Block::with_data(1, &[1, 2, 3]);
        assert_eq!(&b.data[0..3], &[1, 2, 3]);
        assert_eq!(b.data[3], 0);
    }

    #[test]
    fn write_read_roundtrip() {
        let mut b = Block::new(0);
        b.write_at(10, &[0xAA, 0xBB, 0xCC]).unwrap();
        let read = b.read_at(10, 3);
        assert_eq!(read, vec![0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn write_out_of_bounds() {
        let mut b = Block::new(0);
        assert!(b.write_at(BLOCK_SIZE as usize, &[1]).is_err());
    }

    #[test]
    fn partial_write() {
        let mut b = Block::new(0);
        let written = b.write_at(BLOCK_SIZE as usize - 2, &[1, 2, 3, 4]).unwrap();
        assert_eq!(written, 2);
    }

    #[test]
    fn used_bytes() {
        let b = Block::with_data(0, &[1, 2, 3]);
        assert_eq!(b.used_bytes(), 3);
    }
}
