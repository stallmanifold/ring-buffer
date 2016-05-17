use std::io;


pub enum RingBufferError<'a> {
    NotEnoughSpace(usize, usize, &'a str),
    NotEnoughData(usize, usize, &'a str),
    WriteFailure,
    ReadFailure,
}


pub struct RingBuffer {
    buffer: Box<Vec<u8>>,
    capacity: usize,
    start:  usize,
    end:    usize,    
}

impl RingBuffer {
    fn new(capacity: usize) -> RingBuffer {
        let mut buffer = Box::new(Vec::with_capacity(capacity));
        // Initialize vector.
        for i in 0..buffer.capacity() {
            buffer.push(0);
        }

        RingBuffer {
            buffer: buffer,
            capacity: capacity,
            start: 0,
            end: capacity-1,
        }
    }

    fn read_buf(&mut self, target: &mut [u8], amount: usize) -> Result<usize, RingBufferError> {
        unimplemented!();
    }

    fn write_buf(&mut self, data: &[u8], amount: usize) -> Result<usize, RingBufferError> {
        if self.available_data() == 0 {
            // Reset buffer
            self.start = 0;
            self.end = 0;
        }

        if amount > self.available_space() {
            let space = self.available_space();

            return Err(RingBufferError::NotEnoughSpace(space, amount, ""));
        }

        for i in 1..amount+1 {
            self.buffer[(self.end + i) % self.capacity] = data[i];
        }

        self.commit_write(amount);

        Ok(amount)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.available_data() == 0
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.available_data() == self.capacity
    }

    #[inline]
    fn available_data(&self) -> usize {
        (self.end + 1) % self.capacity - self.start - 1
    }

    #[inline]
    fn available_space(&self) -> usize {
        self.capacity - self.end - 1
    }

    fn gets(&mut self, amount: usize) -> Result<Box<[u8]>, RingBufferError> {
        if amount > self.available_data() {
            return Err(RingBufferError::NotEnoughData(amount, self.available_data(), ""));
        }

        let mut result = Vec::new();
        for i in 0..amount {
            result.push(self.buffer[(self.start + i) % self.capacity]);
        }
        assert!(result.len() == amount);

        self.commit_read(amount);
        assert!(self.available_data() >= 0, "Error in read commit.");

        Ok(result.into_boxed_slice())
    }

    fn puts(&mut self, data: &[u8]) -> Result<usize, RingBufferError> {
        self.write_buf(data, data.len())
    }

    #[inline]
    fn commit_read(&mut self, amount: usize) {
        self.start = (self.start + amount) % self.capacity
    }

    #[inline]
    fn commit_write(&mut self, amount: usize) {
        self.end = (self.end + amount) % self.capacity
    }

}