extern crate ring_buffer;

use ring_buffer::RingBuffer;
use std::fmt::Write;


/// GIVEN: An ring buffer.
/// WHEN: It reports being empty.
/// THEN: It should be empty.
#[test]
fn empty_ring_buffer_should_be_empty() {
    let ring_buffer = RingBuffer::new([0x00; 16]);

    assert!(ring_buffer.is_empty());
}

/// GIVEN: A ring buffer.
/// WHEN: The ring buffer is empty.
/// THEN: `extract()` should return an empty slice.
#[test]
fn empty_ring_buffer_extract_should_be_empty_slice() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);

    let result = ring_buffer.extract();
    let expected = "";

    assert_eq!(result, expected);

}

/// GIVEN: An empty ring buffer.
/// WHEN: It reports the length in bytes.
/// THEN: The length should be zero.
#[test]
fn empty_ring_buffer_should_have_length_zero() {
    let ring_buffer = RingBuffer::new([0x00; 16]);

    assert_eq!(ring_buffer.len(), 0);
}

/// GIVEN: A ring buffer.
/// WHEN: It reports it capacity.
/// THEN: Its capacity should be equal to the size of the underlying storage.
#[test]
fn ring_buffer_should_have_capacity_equal_to_underlying_storage_size() {
    let storage = [0xFF as u8; 32];
    let ring_buffer = RingBuffer::new(storage);

    assert_eq!(ring_buffer.capacity(), storage.len());
}

/// GIVEN: A ring buffer and a string to insert of length <= buffer size.
/// WHEN: We call `extract()`.
/// THEN: `extract()` should return the exact same string.
#[test]
fn ring_buffer_extracted_string_should_match_inserted_string_of_length_at_most_the_buffer_size() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnop").unwrap();

    let result = ring_buffer.extract();
    let expected = "abcdefghijklmnop";

    assert_eq!(result, expected);
}

/// GIVEN: A filled ring buffer.
/// WHEN: we call `is_full`.
/// THEN: It should return true.
#[test]
fn ring_buffer_with_string_equal_to_length_in_bytes_should_be_full() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnop").unwrap();

    assert!(ring_buffer.is_full());
}

/// GIVEN: A ring buffer containing data.
/// WHEN: It reports whether it is empty.
/// THEN: It should not be empty.
#[test]
fn ring_buffer_containing_data_should_not_be_empty() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnop").unwrap();

    assert!(!ring_buffer.is_empty());
}

/// GIVEN: A ring buffer containing data.
/// WHEN: We call ``.
/// THEN: Its length should be no larger than the capacity of the buffer.
#[test]
fn ring_buffer_containing_data_should_have_length_no_larger_than_capacity() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklm").unwrap();

    assert!(ring_buffer.len() <= ring_buffer.capacity());
}

/// GIVEN: A ring buffer and a string input longer than the buffer size.
/// WHEN: We write the string to the buffer.
/// THEN: The last buffer length worth of bytes in the string should be present.
#[test]
fn ring_buffer_string_longer_than_buffer_length_should_wrap_around() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnopqrstuv").unwrap();

    let expected = "ghijklmnopqrstuv";
    let result = ring_buffer.extract();

    assert_eq!(result, expected);
}

/// GIVEN: A ring buffer
/// WHEN: We insert many rounds of data into it.
/// THEN: It should return the last buffer length worth of bytes written to it.
#[test]
fn ring_buffer_should_correctly_extract_data_after_multiple_cycles() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnop").unwrap();
    write!(ring_buffer, "abcdefghijklmnop").unwrap();
    write!(ring_buffer, "abcdefghijklmnop").unwrap();
    write!(ring_buffer, "abcdefghijklmnopqrstuv").unwrap();

    let expected = "ghijklmnopqrstuv";
    let result = ring_buffer.extract();

    assert_eq!(result, expected);
}

/// GIVEN: A ring buffer and a string input longer than the buffer size.
/// WHEN: We write the string to the buffer.
/// THEN: The last buffer length worth of bytes in the string should be present.
#[test]
fn ring_buffer_should_only_contain_the_last_buffer_length_number_of_bytes_put_into_it() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnopqrstuvwxyz1234567890").unwrap();

    let expected = "uvwxyz1234567890";
    let result = ring_buffer.extract();

    assert_eq!(result, expected);
}

/// GIVEN: A ring buffer with data.
/// WHEN: We clear the buffer.
/// THEN: The buffer should be empty.
#[test]
fn ring_buffer_should_be_empty_after_clear() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnop").unwrap();
    ring_buffer.clear();

    assert!(ring_buffer.is_empty());
}

/// GIVEN: A ring buffer containing data.
/// WHEN: We call `extract()` after clearing the buffer.
/// THEN: The string should be empty.
#[test]
fn ring_buffer_extract_after_clear_should_be_empty_string() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnop").unwrap();
    ring_buffer.clear();
    let result = ring_buffer.extract();
    let expected = "";

    assert_eq!(result, expected);
}

/// GIVEN: A ring buffer containing data.
/// WHEN: We call `extract()` multiple times in succession.
/// THEN: Each call to `extract()` should return the same string.
#[test]
fn ring_buffer_extract_after_extract_should_yield_same_string() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmn").unwrap();

    let result = String::from(ring_buffer.extract());
    let expected = String::from(ring_buffer.extract());

    assert_eq!(result, expected);
}

/// GIVEN: A filled ring buffer.
/// WHEN: We call `space_remaining()`.
/// THEN: There should be no space remaining.
#[test]
fn filled_ring_buffer_should_have_no_space_remaining() {
    let mut ring_buffer = RingBuffer::new([0x00; 16]);
    write!(ring_buffer, "abcdefghijklmnop").unwrap();

    assert_eq!(ring_buffer.space_remaining(), 0);
}

/// GIVEN: An empty ring buffer.
/// WHEN: We call `space_remaining()`.
/// THEN: The space remaining should be equal to the capacity of the buffer.
#[test]
fn empty_ring_buffer_should_have_maximum_space_remaining() {
    let ring_buffer = RingBuffer::new([0x00; 16]);

    assert_eq!(ring_buffer.space_remaining(), ring_buffer.capacity());
}
