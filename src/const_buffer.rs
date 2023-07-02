use core::slice::ChunksExact;

pub(crate) struct Buffer<const SIZE: usize> {
    data: [u8; SIZE],
    pub(crate) len: usize,
}

impl<const SIZE: usize> Default for Buffer<SIZE> {
    fn default() -> Self {
        Self {
            data: [0; SIZE],
            len: 0,
        }
    }
}

impl<const SIZE: usize> Buffer<SIZE> {
    #[inline(always)]
    pub(crate) fn consume<'a>(&mut self, data: &'a [u8]) -> &'a [u8] {
        let actual_len = core::cmp::min(self.available(), data.len());
        let (data, remaining) = data.split_at(actual_len);
        self.data[self.len..][..actual_len].copy_from_slice(data);
        self.len += actual_len;
        remaining
    }

    #[inline(always)]
    pub(crate) const fn available(&self) -> usize {
        SIZE - self.len
    }

    #[inline(always)]
    pub(crate) const fn is_full(&self) -> bool {
        self.len == SIZE
    }

    #[inline(always)]
    pub(crate) const fn data(&self) -> &[u8; SIZE] {
        &self.data
    }
}
