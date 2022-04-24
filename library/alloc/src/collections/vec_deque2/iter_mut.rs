use core::fmt;
use core::iter::{FusedIterator, TrustedLen, TrustedRandomAccess, TrustedRandomAccessNoCoerce};
use core::marker::PhantomData;

use super::{count, RingSlices, Counter};

/// A mutable iterator over the elements of a `VecDeque2`.
///
/// This `struct` is created by the [`iter_mut`] method on [`super::VecDeque2`]. See its
/// documentation for more.
///
/// [`iter_mut`]: super::VecDeque2::iter_mut
#[stable(feature = "rust1", since = "1.0.0")]
pub struct IterMut<'a, T: 'a> {
    // Internal safety invariant: the entire slice is dereferenceable.
    ring: *mut [T],
    tail: Counter,
    head: Counter,
    phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> IterMut<'a, T> {
    pub(super) unsafe fn new(
        ring: *mut [T],
        tail: Counter,
        head: Counter,
        phantom: PhantomData<&'a mut [T]>,
    ) -> Self {
        IterMut { ring, tail, head, phantom }
    }
}

// SAFETY: we do nothing thread-local and there is no interior mutability,
// so the usual structural `Send`/`Sync` apply.
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Send> Send for IterMut<'_, T> {}
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync> Sync for IterMut<'_, T> {}

#[stable(feature = "collection_debug", since = "1.17.0")]
impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (front, back) = RingSlices::ring_slices(self.ring, self.head, self.tail);
        // SAFETY: these are the elements we have not handed out yet, so aliasing is fine.
        // The `IterMut` invariant also ensures everything is dereferenceable.
        let (front, back) = unsafe { (&*front, &*back) };
        f.debug_tuple("IterMut").field(&front).field(&back).finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        if self.tail == self.head {
            return None;
        }
        let tail = self.tail.to_index(self.ring.len());
        self.tail = self.tail.advance(1).wrapped_for_storage(self.ring.len());

        unsafe {
            let elem = self.ring.get_unchecked_mut(tail);
            Some(&mut *elem)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = count(self.tail, self.head, self.ring.len());
        (len, Some(len))
    }

    fn fold<Acc, F>(self, mut accum: Acc, mut f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        let (front, back) = RingSlices::ring_slices(self.ring, self.head, self.tail);
        // SAFETY: these are the elements we have not handed out yet, so aliasing is fine.
        // The `IterMut` invariant also ensures everything is dereferenceable.
        let (front, back) = unsafe { (&mut *front, &mut *back) };
        accum = front.iter_mut().fold(accum, &mut f);
        back.iter_mut().fold(accum, &mut f)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= count(self.tail, self.head, self.ring.len()) {
            self.tail = self.head;
            None
        } else {
            self.tail = self.tail.advance(n).wrapped_for_storage(self.ring.len());
            self.next()
        }
    }

    #[inline]
    fn last(mut self) -> Option<&'a mut T> {
        self.next_back()
    }

    #[inline]
    #[doc(hidden)]
    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        // Safety: The TrustedRandomAccess contract requires that callers only pass an index
        // that is in bounds.
        unsafe {
            let idx = self.tail.advance(idx).to_index(self.ring.len());
            &mut *self.ring.get_unchecked_mut(idx)
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        if self.tail == self.head {
            return None;
        }
        self.head = self.head.advance_back(1).wrapped_for_storage(self.ring.len());

        unsafe {
            let elem = self.ring.get_unchecked_mut(self.head.to_index(self.ring.len()));
            Some(&mut *elem)
        }
    }

    fn rfold<Acc, F>(self, mut accum: Acc, mut f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        let (front, back) = RingSlices::ring_slices(self.ring, self.head, self.tail);
        // SAFETY: these are the elements we have not handed out yet, so aliasing is fine.
        // The `IterMut` invariant also ensures everything is dereferenceable.
        let (front, back) = unsafe { (&mut *front, &mut *back) };
        accum = back.iter_mut().rfold(accum, &mut f);
        front.iter_mut().rfold(accum, &mut f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for IterMut<'_, T> {
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for IterMut<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for IterMut<'_, T> {}

#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<T> TrustedRandomAccess for IterMut<'_, T> {}

#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<T> TrustedRandomAccessNoCoerce for IterMut<'_, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
