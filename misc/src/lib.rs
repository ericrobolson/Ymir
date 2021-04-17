#![no_std]

pub mod http;

mod parser {
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum StreamError {
        BufferOverflow {
            max_bytes: usize,
            actual_size: usize,
        },
    }

    /// Struct for iterating over a raw request
    pub struct RawIter<'a> {
        data: &'a [u8],
        index: usize,
    }

    impl<'a> RawIter<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            Self { index: 0, data }
        }

        /// Immutable handle to the data
        pub fn data(&self) -> &[u8] {
            &self.data[self.index..]
        }

        /// Advance the iterator by the given length
        pub fn advance(&mut self, len: usize) {
            self.index += len;
        }

        /// Get the index of where the iterator is currently at
        pub fn index(&self) -> usize {
            self.index
        }
    }
}
