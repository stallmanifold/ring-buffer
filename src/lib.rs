use std::fmt;
use std::default::Default;
use std::marker::PhantomData;
use std::vec::Vec;


pub trait BufferStorage<T>: AsRef<[T]> + AsMut<[T]> where T: Sized {}

impl<T> BufferStorage<T> for [T; 1] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 2] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 3] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 4] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 5] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 6] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 7] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 8] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 9] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 10] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 11] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 12] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 13] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 14] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 15] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 16] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 17] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 18] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 19] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 20] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 21] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 22] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 23] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 24] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 25] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 26] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 27] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 28] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 29] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 30] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 31] where T: Sized + Default {}
impl<T> BufferStorage<T> for [T; 32] where T: Sized + Default {}

impl<T> BufferStorage<T> for Vec<T> where T: Sized + Default {}

/// A `RingBuffer` is a ring buffer providing the illusion of storing an 
/// unlimited stream of sized objects of a given type in a finite amount 
/// of space. It stores data using zero allocations.
#[derive(Debug)]
pub struct RingBuffer<S, T> {
    /// The underlying storage for the ring buffer.
    storage: S,
    /// Whether the ring buffer has wrapped around since its last call to rotate.
    wrapped: bool,
    /// The position of the next available byte in the ring buffer.
    end: usize,
    _marker: PhantomData<T>,
}

impl<S, T> RingBuffer<S, T> where S: BufferStorage<T>, T: Copy + Sized + Default {
    /// Construct a new ring buffer.
    pub fn new(storage: S) -> RingBuffer<S, T> {
        let mut ring_buffer = RingBuffer {
            storage: storage,
            wrapped: false,
            end: 0,
            _marker: PhantomData
        };

        ring_buffer.clear();
        ring_buffer
    }

    /// Empty out the ring buffer.
    pub fn clear(&mut self) {
        self.wrapped = false;
        self.end = 0;
        for slot in self.storage.as_mut().iter_mut() {
            *slot = T::default();
        }
    }

    /// Determine whether the ring buffer is empty, i.e. it contains no data.
    pub fn is_empty(&self) -> bool {
        (self.end == 0) && !self.wrapped
    }

    /// Determine the number of items currently stored in the ring buffer.
    pub fn len(&self) -> usize {
        if self.wrapped {
            self.storage.as_ref().len()
        } else {
            self.end
        }
    }

    /// Calculate the amount of space in number of items remaining in the ring buffer.
    pub fn space_remaining(&self) -> usize {
        self.capacity() - self.len()
    }

    /// Determine whether the ring buffer is full.
    pub fn is_full(&self) -> bool {
        self.space_remaining() == 0
    }

    /// The maximum number of items that a ring buffer can store.
    pub fn capacity(&self) -> usize {
        self.storage.as_ref().len()
    }

    fn rotate(&mut self) {
        if self.wrapped {
            self.storage.as_mut().rotate_left(self.end);
            self.end = self.len();
            self.wrapped = false;
        }
    }

    /// Extract a slice from the ring buffer. This is a zero allocation operation.
    pub fn extract(&mut self) -> &[T] {
        self.rotate();

        let buffer = self.storage.as_mut();
        let end = self.end;
        &buffer[0..end]
    }

    /// Write a slice of objects into the ring buffer.
    pub fn write(&mut self, slice: &[T]) -> Result<(), ()> {
        for &item in slice.iter() {
            self.storage.as_mut()[self.end] = item;
            self.end += 1;
            if self.end >= self.storage.as_ref().len() {
                self.wrapped = true;
            }
            self.end %= self.storage.as_mut().len();
        }

        Ok(())
    }
}

impl<S> fmt::Write for RingBuffer<S, u8> where S: BufferStorage<u8> {
    /// Write a UTF-8 string into the ring buffer.
    fn write_str(&mut self, st: &str) -> fmt::Result {
        for &byte in st.as_bytes() {
            self.storage.as_mut()[self.end] = byte;
            self.end += 1;
            if self.end >= self.storage.as_ref().len() {
                self.wrapped = true;
            }
            self.end %= self.storage.as_mut().len();
        }

        Ok(())
    }
}


#[cfg(test)]
mod rotate_tests {
    use super::RingBuffer;
    use core::fmt::Write;


    #[test]
    fn ring_buffer_successive_rotate_operations_should_leave_internal_state_unchanged() {
        let mut ring_buffer = RingBuffer::new([0xFF; 16]);
        write!(ring_buffer, "abcdefgh").unwrap();

        ring_buffer.rotate();
        let wrapped = ring_buffer.wrapped;
        let end = ring_buffer.end;
        let storage = ring_buffer.storage;
        ring_buffer.rotate();
        ring_buffer.rotate();

        assert_eq!(ring_buffer.wrapped, wrapped);
        assert_eq!(ring_buffer.end, end);
        assert_eq!(ring_buffer.storage, storage);
    }

    #[test]
    fn ring_buffer_rotate_should_unwrap_buffer() {
        let mut ring_buffer = RingBuffer::new([0xFF; 16]);
        write!(ring_buffer, "abcdefghijklmnop").unwrap();

        assert_eq!(ring_buffer.wrapped, true);
        ring_buffer.rotate();
        assert_eq!(ring_buffer.wrapped, false);
    }

    #[test]
    fn ring_buffer_rotate_should_unwrap_end() {
        let mut ring_buffer = RingBuffer::new([0xFF; 16]);
        write!(ring_buffer, "abcdefghijklmnop").unwrap();

        let end_before_rotate = ring_buffer.end;
        ring_buffer.rotate();
        assert!(ring_buffer.end > end_before_rotate);
    }
}

