use core::ptr::{self};
use super::{Counter, is_contiguous, log, count};

/// Returns the two slices that cover the `VecDeque2`'s valid range
pub(crate) trait RingSlices: Sized {
    fn slice(self, from: usize, to: usize) -> Self;
    fn split_at(self, i: usize) -> (Self, Self);
    fn length(&self) -> usize;

    fn ring_slices(buf: Self, head: Counter, tail: Counter) -> (Self, Self) {
        let contiguous = is_contiguous(head, tail, buf.length());
        let wrapped_tail = tail.to_index(buf.length());
        if contiguous {
            let (empty, buf) = buf.split_at(0);
            let length = count(tail, head, buf.length());
            (buf.slice(wrapped_tail, wrapped_tail + length), empty)
        } else {
            let wrapped_head = head.to_index(buf.length());
            let (mid, right) = buf.split_at(wrapped_tail);
            let (left, _) = mid.split_at(wrapped_head);
            (right, left)
        }
    }
}

impl<T> RingSlices for &[T] {
    fn slice(self, from: usize, to: usize) -> Self {
        &self[from..to]
    }
    fn split_at(self, i: usize) -> (Self, Self) {
        (*self).split_at(i)
    }
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> RingSlices for &mut [T] {
    fn slice(self, from: usize, to: usize) -> Self {
        &mut self[from..to]
    }
    fn split_at(self, i: usize) -> (Self, Self) {
        (*self).split_at_mut(i)
    }
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> RingSlices for *mut [T] {
    fn slice(self, from: usize, to: usize) -> Self {
        assert!(from <= to && to < self.len());
        // Not using `get_unchecked_mut` to keep this a safe operation.
        let len = to - from;
        ptr::slice_from_raw_parts_mut(self.as_mut_ptr().wrapping_add(from), len)
    }

    fn split_at(self, mid: usize) -> (Self, Self) {
        let len = self.len();
        let ptr = self.as_mut_ptr();
        assert!(mid <= len);
        (
            ptr::slice_from_raw_parts_mut(ptr, mid),
            ptr::slice_from_raw_parts_mut(ptr.wrapping_add(mid), len - mid),
        )
    }
    fn length(&self) -> usize {
        self.len()
    }
}
