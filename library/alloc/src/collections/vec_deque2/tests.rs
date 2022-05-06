use super::*;
use crate::string::ToString;

fn check_vec<T: std::fmt::Debug + std::cmp::Eq + Copy>(deque: &VecDeque2<T>, expected: Vec<T>) {
    assert_eq!(deque.len(), expected.len());
    let mut it1 = deque.iter();
    let mut it2 = expected.iter();
    loop {
        let a = it1.next();
        let b = it2.next();
        assert_eq!(a, b);
        if b.is_none() {
            break;
        }
    }

    assert_eq!(deque.iter().copied().collect::<Vec<_>>(), expected);
}

// Try to double the capacity of the queue and check that it has not interfered with its values
// and length.
fn check_grow<T: std::fmt::Debug + std::cmp::Eq + Copy>(
    deque: &mut VecDeque2<T>,
    expected: Vec<T>
) {
    check_vec(&deque, expected.clone());
    deque.reserve_exact(deque.capacity());
    check_vec(&deque, expected);
}

fn to_str<T: std::fmt::Display>(deque: &VecDeque2<T>) -> std::string::String {
    let capacity = deque.capacity();
    let mut indices = vec!["_".to_string(); capacity];
    indices[deque.head.to_index(capacity)].insert(0, 'h');
    indices[deque.tail.to_index(capacity)].insert(0, 't');

    let wrapped_tail = deque.tail.to_index(capacity);
    for index in 0..deque.len() {
        let target_index = (wrapped_tail + index) % capacity;
        let value_ref = unsafe { deque.ptr().add(target_index).as_ref().unwrap() };
        let string = indices.get_mut(target_index).unwrap();
        string.pop();
        string.push_str(&value_ref.to_string());
    }

    for string in indices.iter_mut() {
        if string.len() > 1 && string.ends_with('_') {
            string.pop();
        }
    }

    let mut output = indices.join(",");
    output += "|";
    for index in 0..capacity {
        let mut has_counter = false;
        if deque.tail.0 == capacity + index {
            output.push('T');
            has_counter = true;
        }
        if deque.head.0 == capacity + index {
            output.push('H');
            has_counter = true;
        }
        if !has_counter {
            output.push('_');
        }
        if index != capacity - 1 {
            output.push(',');
        }
    }

    output
}

#[test]
fn vd2_test_empty_drop() {
    let _tester: VecDeque2<u64> = VecDeque2::new();
}

#[test]
fn vd2_test_empty_iter_zero_capacity() {
    let tester: VecDeque2<u64> = VecDeque2::new();
    assert!(tester.iter().next() == None);
    check_vec(&tester, vec!());
}

#[test]
fn vd2_test_empty_iter_nonzero_capacity() {
    let tester: VecDeque2<u64> = VecDeque2::with_capacity(16);
    assert!(tester.iter().next() == None);
    check_vec(&tester, vec!());
}

#[test]
fn vd2_test_is_empty_1() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    assert_eq!(tester.len(), 0);
    assert!(tester.is_empty());
    tester.push_back(1);
    assert_eq!(tester.len(), 1);
    assert!(!tester.is_empty());
}

#[test]
fn vd2_test_is_empty_2() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    assert_eq!(tester.len(), 0);
    assert!(tester.is_empty());
    tester.push_front(1);
    assert_eq!(tester.len(), 1);
    assert!(!tester.is_empty());
}

#[test]
fn vd2_test_pop_back() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    assert_eq!(tester.len(), 0);
    tester.push_back(1);
    tester.push_back(2);
    tester.pop_back();
    tester.push_back(3);
    tester.pop_back();

    assert_eq!(tester.len(), 1);

    assert_eq!(tester.get(0), Some(&1));

    tester.pop_back();
    assert_eq!(tester.len(), 0);
}

#[test]
fn vd2_test_pop_front() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    assert_eq!(tester.capacity(), 0);
    tester.push_front(6);
    assert_eq!(tester.capacity(), 1);
    tester.push_back(1);
    assert_eq!(tester.capacity(), 2);
    tester.push_back(3);
    assert_eq!(tester.capacity(), 4);
    tester.push_back(5);

    assert_eq!(tester.capacity(), 4);
    assert_eq!(tester.len(), 4);

    assert_eq!(tester.pop_front(), Some(6));
    assert_eq!(tester.pop_front(), Some(1));
    assert_eq!(tester.pop_front(), Some(3));
    assert_eq!(tester.pop_front(), Some(5));
    assert_eq!(tester.pop_front(), None);
}

#[test]
fn vd2_test_as_slices_1() {
    let mut tester = VecDeque2::with_capacity(4);
    tester.push_back(0);
    tester.push_back(1);
    tester.push_back(2);

    let (a, b) = tester.as_slices();
    assert_eq!(a, [0, 1, 2]);
    assert_eq!(b, []);
}

#[test]
fn vd2_test_as_slices_2() {
    let mut tester = VecDeque2::with_capacity(4);
    tester.push_front(0);
    tester.push_back(1);
    tester.push_back(2);

    let (a, b) = tester.as_slices();
    assert_eq!(a, [0]);
    assert_eq!(b, [1, 2]);
}

#[test]
fn vd2_test_as_slices_3() {
    let mut tester: VecDeque2<u64> = VecDeque2::with_capacity(4);
    tester.push_front(0);
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    assert_eq!(to_str(&tester), "1,2,3,th0|_,_,_,T");
    let (a, b) = tester.as_slices();
    assert_eq!(a, [0]);
    assert_eq!(b, [1, 2, 3]);
}

#[test]
fn vd2_test_iter() {
    let mut tester = VecDeque2::with_capacity(4);
    tester.push_back(0);
    tester.push_back(1);
    tester.push_back(2);

    assert_eq!(tester.iter().copied().collect::<Vec<_>>(), vec![0, 1, 2]);
}

#[test]
fn vd2_test_iter_full() {
    let mut tester = VecDeque2::with_capacity(4);
    tester.push_back(0);
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    check_vec(&tester, vec![0, 1, 2, 3]);
}

// Tests that check if reallocation of the VecDeque2 was done properly.
#[test]
fn vd2_test_grow_t_h_in_bounds() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);

    assert_eq!(to_str(&vd), "t1,2,3,h|_,_,_,_");
    check_grow(&mut vd, vec![1, 2, 3]);
    assert_eq!(to_str(&vd), "t1,2,3,h,_,_,_,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_grow_t_h_overflow() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(5);
    vd.push_back(5);
    vd.push_back(5);
    vd.pop_front();
    vd.pop_front();
    vd.pop_front();
    vd.push_back(5);
    vd.pop_front();
    vd.push_back(1);

    assert_eq!(to_str(&vd), "t1,h,_,_|T,H,_,_");
    check_grow(&mut vd, vec![1]);
    assert_eq!(to_str(&vd), "t1,h,_,_,_,_,_,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_grow_h_t_nothing_to_copy() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(5);
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);
    vd.pop_front();

    assert_eq!(to_str(&vd), "h,t1,2,3|H,_,_,_");
    check_grow(&mut vd, vec![1, 2, 3]);
    assert_eq!(to_str(&vd), "_,t1,2,3,h,_,_,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_grow_h_t_copy_after_t() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(5);
    vd.push_back(5);
    vd.pop_front();
    vd.pop_front();
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);

    assert_eq!(to_str(&vd), "3,h,t1,2|_,H,_,_");
    check_grow(&mut vd, vec![1, 2, 3]);
    assert_eq!(to_str(&vd), "_,_,t1,2,3,h,_,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_grow_h_t_copy_move_t() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(5);
    vd.push_back(5);
    vd.push_back(5);
    vd.pop_front();
    vd.pop_front();
    vd.pop_front();
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);

    assert_eq!(to_str(&vd), "2,3,h,t1|_,_,H,_");
    check_grow(&mut vd, vec![1, 2, 3]);
    assert_eq!(to_str(&vd), "2,3,h,_,_,_,_,t1|_,_,_,_,_,_,_,T");
}

#[test]
fn vd2_test_grow_full_beginning_1() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);
    vd.push_back(4);

    assert_eq!(to_str(&vd), "th1,2,3,4|H,_,_,_");
    check_grow(&mut vd, vec![1, 2, 3, 4]);
    assert_eq!(to_str(&vd), "t1,2,3,4,h,_,_,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_grow_full_beginning_2() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_front(4);
    vd.push_front(3);
    vd.push_front(2);
    vd.push_front(1);

    assert_eq!(to_str(&vd), "th1,2,3,4|T,_,_,_");
    check_grow(&mut vd, vec![1, 2, 3, 4]);
    assert_eq!(to_str(&vd), "t1,2,3,4,h,_,_,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_grow_full_end_1() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(5);
    vd.push_back(5);
    vd.push_back(5);
    vd.pop_front();
    vd.pop_front();
    vd.pop_front();
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);
    vd.push_back(4);

    assert_eq!(to_str(&vd), "2,3,4,th1|_,_,_,H");
    check_grow(&mut vd, vec![1, 2, 3, 4]);
    assert_eq!(to_str(&vd), "2,3,4,h,_,_,_,t1|_,_,_,_,_,_,_,T");
}

#[test]
fn vd2_test_grow_full_end_2() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_front(1);
    vd.push_back(2);
    vd.push_back(3);
    vd.push_back(4);

    assert_eq!(to_str(&vd), "2,3,4,th1|_,_,_,T");
    check_grow(&mut vd, vec![1, 2, 3, 4]);
    assert_eq!(to_str(&vd), "2,3,4,h,_,_,_,t1|_,_,_,_,_,_,_,T");
}

#[test]
fn vd2_test_grow_full_middle_copy_after_t_1() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_front(3);
    vd.push_front(2);
    vd.push_front(1);
    vd.push_back(4);

    assert_eq!(to_str(&vd), "4,th1,2,3|_,T,_,_");
    check_grow(&mut vd, vec![1, 2, 3, 4]);
    assert_eq!(to_str(&vd), "_,t1,2,3,4,h,_,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_grow_full_middle_copy_after_t_2() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(5);
    vd.pop_front();
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);
    vd.push_back(4);

    assert_eq!(to_str(&vd), "4,th1,2,3|_,H,_,_");
    check_grow(&mut vd, vec![1, 2, 3, 4]);
    assert_eq!(to_str(&vd), "_,t1,2,3,4,h,_,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_grow_full_middle_move_t_1() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    for _ in 0..5 {
        vd.push_back(5);
    }
    for _ in 0..5 {
        vd.pop_front();
    }

    for i in 1..=8 {
        vd.push_back(i);
    }

    assert_eq!(to_str(&vd), "4,5,6,7,8,th1,2,3|_,_,_,_,_,H,_,_");
    check_grow(&mut vd, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(to_str(&vd), "4,5,6,7,8,h,_,_,_,_,_,_,_,t1,2,3|_,_,_,_,_,_,_,_,_,_,_,_,_,T,_,_");
}

#[test]
fn vd2_test_grow_full_middle_move_t_2() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    vd.push_front(3);
    vd.push_front(2);
    vd.push_front(1);
    vd.push_back(4);
    vd.push_back(5);
    vd.push_back(6);
    vd.push_back(7);
    vd.push_back(8);

    assert_eq!(to_str(&vd), "4,5,6,7,8,th1,2,3|_,_,_,_,_,T,_,_");
    check_grow(&mut vd, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(to_str(&vd), "4,5,6,7,8,h,_,_,_,_,_,_,_,t1,2,3|_,_,_,_,_,_,_,_,_,_,_,_,_,T,_,_");
}

#[test]
fn vd2_test_remove_contiguous_closer_to_tail() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    for i in 0..8 {
        vd.push_back(i);
    }

    assert_eq!(to_str(&vd), "th0,1,2,3,4,5,6,7|H,_,_,_,_,_,_,_");
    vd.remove(2);
    assert_eq!(to_str(&vd), "h,t0,1,3,4,5,6,7|H,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_remove_contiguous_closer_to_head() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    for i in 0..8 {
        vd.push_back(i);
    }

    assert_eq!(to_str(&vd), "th0,1,2,3,4,5,6,7|H,_,_,_,_,_,_,_");
    vd.remove(6);
    assert_eq!(to_str(&vd), "t0,1,2,3,4,5,7,h|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_remove_discontiguous_closer_to_tail_in_tail() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    vd.push_front(1);
    vd.push_front(0);
    vd.push_back(2);
    vd.push_back(3);
    vd.push_back(4);

    assert_eq!(to_str(&vd), "2,3,4,h,_,_,t0,1|_,_,_,_,_,_,T,_");
    vd.remove(1);
    assert_eq!(to_str(&vd), "2,3,4,h,_,_,_,t0|_,_,_,_,_,_,_,T");
}

#[test]
fn vd2_test_remove_discontiguous_closer_to_head_in_head() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    vd.push_front(1);
    vd.push_front(0);
    vd.push_back(2);
    vd.push_back(3);
    vd.push_back(4);

    assert_eq!(to_str(&vd), "2,3,4,h,_,_,t0,1|_,_,_,_,_,_,T,_");
    vd.remove(3);
    assert_eq!(to_str(&vd), "2,4,h,_,_,_,t0,1|_,_,_,_,_,_,T,_");
}

#[test]
fn vd2_test_remove_discontiguous_closer_to_head_in_tail() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    vd.push_front(3);
    vd.push_front(2);
    vd.push_front(1);
    vd.push_front(0);
    vd.push_back(4);

    assert_eq!(to_str(&vd), "4,h,_,_,t0,1,2,3|_,_,_,_,T,_,_,_");
    vd.remove(3);
    assert_eq!(to_str(&vd), "h,_,_,_,t0,1,2,4|_,_,_,_,T,_,_,_");
}

#[test]
fn vd2_test_remove_discontiguous_closer_to_tail_in_head() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(16);
    vd.push_front(2);
    vd.push_front(1);
    vd.push_front(0);
    for i in 3..10 {
        vd.push_back(i);
    }

    assert_eq!(to_str(&vd), "3,4,5,6,7,8,9,h,_,_,_,_,_,t0,1,2|_,_,_,_,_,_,_,_,_,_,_,_,_,T,_,_");
    vd.remove(5);
    assert_eq!(to_str(&vd), "2,3,4,6,7,8,9,h,_,_,_,_,_,_,t0,1|_,_,_,_,_,_,_,_,_,_,_,_,_,_,T,_");
}

#[test]
fn vd2_test_insert_tail_zero() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(4);
    vd.push_back(0);
    vd.push_back(1);

    assert_eq!(to_str(&vd), "t0,1,h,_|_,_,_,_");
    vd.insert(0, 9);
    assert_eq!(to_str(&vd), "0,1,h,t9|_,_,_,T");
}

#[test]
fn vd2_test_insert_contiguous_closer_to_tail() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    for i in 0..8 {
        vd.push_back(i);
    }
    vd.pop_front();

    assert_eq!(to_str(&vd), "h,t1,2,3,4,5,6,7|H,_,_,_,_,_,_,_");
    vd.insert(2, 9);
    assert_eq!(to_str(&vd), "th1,2,9,3,4,5,6,7|H,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_insert_contiguous_closer_to_head() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    for i in 0..8 {
        vd.push_back(i);
    }
    vd.pop_front();

    assert_eq!(to_str(&vd), "h,t1,2,3,4,5,6,7|H,_,_,_,_,_,_,_");
    vd.insert(6, 9);
    assert_eq!(to_str(&vd), "7,th1,2,3,4,5,6,9|_,H,_,_,_,_,_,_");
}

#[test]
fn vd2_test_make_contiguous_a() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(16);
    vd.push_back(3);
    vd.push_back(4);
    vd.push_back(5);
    vd.push_front(2);
    vd.push_front(1);
    vd.push_front(0);

    assert_eq!(to_str(&vd), "3,4,5,h,_,_,_,_,_,_,_,_,_,t0,1,2|_,_,_,_,_,_,_,_,_,_,_,_,_,T,_,_");
    vd.make_contiguous();
    assert_eq!(to_str(&vd), "t0,1,2,3,4,5,h,_,_,_,_,_,_,_,_,_|_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_make_contiguous_b() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    vd.push_back(4);
    vd.push_front(3);
    vd.push_front(2);
    vd.push_front(1);
    vd.push_front(0);

    assert_eq!(to_str(&vd), "4,h,_,_,t0,1,2,3|_,_,_,_,T,_,_,_");
    vd.make_contiguous();
    assert_eq!(to_str(&vd), "_,t0,1,2,3,4,h,_|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_make_contiguous_c() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    vd.push_back(4);
    vd.push_back(5);
    vd.push_back(6);
    vd.push_front(3);
    vd.push_front(2);
    vd.push_front(1);
    vd.push_front(0);

    assert_eq!(to_str(&vd), "4,5,6,h,t0,1,2,3|_,_,_,_,T,_,_,_");
    vd.make_contiguous();
    assert_eq!(to_str(&vd), "t0,1,2,3,4,5,6,h|_,_,_,_,_,_,_,_");
}

#[test]
fn vd2_test_shrink_a() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(8);
    vd.push_front(3);
    vd.push_front(2);
    vd.push_front(1);
    vd.push_front(0);
    vd.pop_back();

    assert_eq!(to_str(&vd), "_,_,_,_,t0,1,2,h|_,_,_,_,T,_,_,H");
    vd.shrink_to(3);
    assert_eq!(vd.capacity(), 4);
    assert_eq!(to_str(&vd), "t0,1,2,h|_,_,_,_");
}

#[test]
fn vd2_test_shrink_b1() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(16);
    for i in 0..10 {
        vd.push_back(i);
    }
    vd.pop_front();
    vd.pop_front();
    vd.pop_front();

    assert_eq!(to_str(&vd), "_,_,_,t3,4,5,6,7,8,9,h,_,_,_,_,_|_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_");
    vd.shrink_to(8);
    assert_eq!(vd.capacity(), 8);
    assert_eq!(to_str(&vd), "8,9,h,t3,4,5,6,7|_,_,H,_,_,_,_,_");
}

#[test]
fn vd2_test_shrink_b2() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(16);
    for i in 0..10 {
        vd.push_back(i);
    }
    vd.pop_front();
    vd.pop_front();

    assert_eq!(to_str(&vd), "_,_,t2,3,4,5,6,7,8,9,h,_,_,_,_,_|_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_");
    vd.shrink_to(8);
    assert_eq!(vd.capacity(), 8);
    assert_eq!(to_str(&vd), "8,9,th2,3,4,5,6,7|_,_,H,_,_,_,_,_");
}

#[test]
fn vd2_test_shrink_c() {
    let mut vd: VecDeque2<u64> = VecDeque2::with_capacity(16);
    vd.push_front(1);
    vd.push_front(2);
    for i in 3..8 {
        vd.push_back(i);
    }

    assert_eq!(to_str(&vd), "3,4,5,6,7,h,_,_,_,_,_,_,_,_,t2,1|_,_,_,_,_,_,_,_,_,_,_,_,_,_,T,_");
    vd.shrink_to(8);
    assert_eq!(vd.capacity(), 8);
    assert_eq!(to_str(&vd), "3,4,5,6,7,h,t2,1|_,_,_,_,_,_,T,_");
}

// PR tests
#[test]
fn vd2_test_clear() {
    let mut tester = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);
    tester.clear();
    assert_eq!(tester.len(), 0);
}

#[test]
fn vd2_test_get() {
    let mut tester = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    assert_eq!(tester.len(), 3);

    assert_eq!(tester.get(1), Some(&2));
    assert_eq!(tester.get(2), Some(&3));
    assert_eq!(tester.get(0), Some(&1));
    assert_eq!(tester.get(3), None);

    tester.remove(0);

    assert_eq!(tester.len(), 2);
    assert_eq!(tester.get(0), Some(&2));
    assert_eq!(tester.get(1), Some(&3));
    assert_eq!(tester.get(2), None);
}

#[test]
fn vd2_test_get_mut() {
    let mut tester = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    assert_eq!(tester.len(), 3);

    if let Some(elem) = tester.get_mut(0) {
        assert_eq!(*elem, 1);
        *elem = 10;
    }

    if let Some(elem) = tester.get_mut(2) {
        assert_eq!(*elem, 3);
        *elem = 30;
    }

    assert_eq!(tester.get(0), Some(&10));
    assert_eq!(tester.get(2), Some(&30));
    assert_eq!(tester.get_mut(3), None);

    tester.remove(2);

    assert_eq!(tester.len(), 2);
    assert_eq!(tester.get(0), Some(&10));
    assert_eq!(tester.get(1), Some(&2));
    assert_eq!(tester.get(2), None);
}

#[test]
fn vd2_test_swap() {
    let mut tester = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    assert_eq!(tester, [1, 2, 3]);

    tester.swap(0, 0);
    assert_eq!(tester, [1, 2, 3]);
    tester.swap(0, 1);
    assert_eq!(tester, [2, 1, 3]);
    tester.swap(2, 1);
    assert_eq!(tester, [2, 3, 1]);
    tester.swap(1, 2);
    assert_eq!(tester, [2, 1, 3]);
    tester.swap(0, 2);
    assert_eq!(tester, [3, 1, 2]);
    tester.swap(2, 2);
    assert_eq!(tester, [3, 1, 2]);
}

#[test]
#[should_panic = "assertion failed: j < self.len()"]
fn vd2_test_swap_panic() {
    let mut tester = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);
    tester.swap(2, 3);
}

#[test]
fn vd2_test_reserve_exact() {
    let mut tester: VecDeque2<i32> = VecDeque2::with_capacity(1);
    assert!(tester.capacity() == 1);
    tester.reserve_exact(50);
    assert!(tester.capacity() >= 51);
    tester.reserve_exact(40);
    assert!(tester.capacity() >= 51);
    tester.reserve_exact(200);
    assert!(tester.capacity() >= 200);
}

#[test]
#[should_panic = "capacity overflow"]
fn vd2_test_reserve_exact_panic() {
    let mut tester: VecDeque2<i32> = VecDeque2::new();
    tester.reserve_exact(usize::MAX);
}

#[test]
fn vd2_test_try_reserve_exact() {
    let mut tester: VecDeque2<i32> = VecDeque2::with_capacity(1);
    assert!(tester.capacity() == 1);
    assert_eq!(tester.try_reserve_exact(100), Ok(()));
    assert!(tester.capacity() >= 100);
    assert_eq!(tester.try_reserve_exact(50), Ok(()));
    assert!(tester.capacity() >= 100);
    assert_eq!(tester.try_reserve_exact(200), Ok(()));
    assert!(tester.capacity() >= 200);
    assert_eq!(tester.try_reserve_exact(0), Ok(()));
    assert!(tester.capacity() >= 200);
    assert!(tester.try_reserve_exact(usize::MAX).is_err());
}

#[test]
fn vd2_test_try_reserve() {
    let mut tester: VecDeque2<i32> = VecDeque2::with_capacity(1);
    assert!(tester.capacity() == 1);
    assert_eq!(tester.try_reserve(100), Ok(()));
    assert!(tester.capacity() >= 100);
    assert_eq!(tester.try_reserve(50), Ok(()));
    assert!(tester.capacity() >= 100);
    assert_eq!(tester.try_reserve(200), Ok(()));
    assert!(tester.capacity() >= 200);
    assert_eq!(tester.try_reserve(0), Ok(()));
    assert!(tester.capacity() >= 200);
    assert!(tester.try_reserve(usize::MAX).is_err());
}

#[test]
fn vd2_test_contains() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    assert!(tester.contains(&1));
    assert!(tester.contains(&3));
    assert!(!tester.contains(&0));
    assert!(!tester.contains(&4));
    tester.remove(0);
    assert!(!tester.contains(&1));
    assert!(tester.contains(&2));
    assert!(tester.contains(&3));
}

#[test]
fn vd2_test_rotate_left_right() {
    let mut tester: VecDeque2<_> = (1..=10).collect();

    assert_eq!(tester.len(), 10);

    tester.rotate_left(0);
    assert_eq!(tester, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    tester.rotate_right(0);
    assert_eq!(tester, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    tester.rotate_left(3);
    assert_eq!(tester, [4, 5, 6, 7, 8, 9, 10, 1, 2, 3]);

    tester.rotate_right(5);
    assert_eq!(tester, [9, 10, 1, 2, 3, 4, 5, 6, 7, 8]);

    tester.rotate_left(tester.len());
    assert_eq!(tester, [9, 10, 1, 2, 3, 4, 5, 6, 7, 8]);

    tester.rotate_right(tester.len());
    assert_eq!(tester, [9, 10, 1, 2, 3, 4, 5, 6, 7, 8]);

    tester.rotate_left(1);
    assert_eq!(tester, [10, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
#[should_panic = "assertion failed: mid <= self.len()"]
fn vd2_test_rotate_left_panic() {
    let mut tester: VecDeque2<_> = (1..=10).collect();
    tester.rotate_left(tester.len() + 1);
}

#[test]
#[should_panic = "assertion failed: k <= self.len()"]
fn vd2_test_rotate_right_panic() {
    let mut tester: VecDeque2<_> = (1..=10).collect();
    tester.rotate_right(tester.len() + 1);
}

#[test]
fn vd2_test_binary_search() {
    // If the givin VecDeque is not sorted, the returned result is unspecified and meaningless,
    // as this method performs a binary search.

    let tester: VecDeque2<_> = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();

    assert_eq!(tester.binary_search(&0), Ok(0));
    assert_eq!(tester.binary_search(&5), Ok(5));
    assert_eq!(tester.binary_search(&55), Ok(10));
    assert_eq!(tester.binary_search(&4), Err(5));
    assert_eq!(tester.binary_search(&-1), Err(0));
    assert!(matches!(tester.binary_search(&1), Ok(1..=2)));

    let tester: VecDeque2<_> = [1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3].into();
    assert_eq!(tester.binary_search(&1), Ok(0));
    assert!(matches!(tester.binary_search(&2), Ok(1..=4)));
    assert!(matches!(tester.binary_search(&3), Ok(5..=13)));
    assert_eq!(tester.binary_search(&-2), Err(0));
    assert_eq!(tester.binary_search(&0), Err(0));
    assert_eq!(tester.binary_search(&4), Err(14));
    assert_eq!(tester.binary_search(&5), Err(14));
}

#[test]
fn vd2_test_binary_search_by() {
    // If the givin VecDeque is not sorted, the returned result is unspecified and meaningless,
    // as this method performs a binary search.

    let tester: VecDeque2<_> = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();

    assert_eq!(tester.binary_search_by(|x| x.cmp(&0)), Ok(0));
    assert_eq!(tester.binary_search_by(|x| x.cmp(&5)), Ok(5));
    assert_eq!(tester.binary_search_by(|x| x.cmp(&55)), Ok(10));
    assert_eq!(tester.binary_search_by(|x| x.cmp(&4)), Err(5));
    assert_eq!(tester.binary_search_by(|x| x.cmp(&-1)), Err(0));
    assert!(matches!(tester.binary_search_by(|x| x.cmp(&1)), Ok(1..=2)));
}

#[test]
fn vd2_test_binary_search_key() {
    // If the givin VecDeque is not sorted, the returned result is unspecified and meaningless,
    // as this method performs a binary search.

    let tester: VecDeque2<_> = [
        (-1, 0),
        (2, 10),
        (6, 5),
        (7, 1),
        (8, 10),
        (10, 2),
        (20, 3),
        (24, 5),
        (25, 18),
        (28, 13),
        (31, 21),
        (32, 4),
        (54, 25),
    ]
    .into();

    assert_eq!(tester.binary_search_by_key(&-1, |&(a, _b)| a), Ok(0));
    assert_eq!(tester.binary_search_by_key(&8, |&(a, _b)| a), Ok(4));
    assert_eq!(tester.binary_search_by_key(&25, |&(a, _b)| a), Ok(8));
    assert_eq!(tester.binary_search_by_key(&54, |&(a, _b)| a), Ok(12));
    assert_eq!(tester.binary_search_by_key(&-2, |&(a, _b)| a), Err(0));
    assert_eq!(tester.binary_search_by_key(&1, |&(a, _b)| a), Err(1));
    assert_eq!(tester.binary_search_by_key(&4, |&(a, _b)| a), Err(2));
    assert_eq!(tester.binary_search_by_key(&13, |&(a, _b)| a), Err(6));
    assert_eq!(tester.binary_search_by_key(&55, |&(a, _b)| a), Err(13));
    assert_eq!(tester.binary_search_by_key(&100, |&(a, _b)| a), Err(13));

    let tester: VecDeque2<_> = [
        (0, 0),
        (2, 1),
        (6, 1),
        (5, 1),
        (3, 1),
        (1, 2),
        (2, 3),
        (4, 5),
        (5, 8),
        (8, 13),
        (1, 21),
        (2, 34),
        (4, 55),
    ]
    .into();

    assert_eq!(tester.binary_search_by_key(&0, |&(_a, b)| b), Ok(0));
    assert!(matches!(tester.binary_search_by_key(&1, |&(_a, b)| b), Ok(1..=4)));
    assert_eq!(tester.binary_search_by_key(&8, |&(_a, b)| b), Ok(8));
    assert_eq!(tester.binary_search_by_key(&13, |&(_a, b)| b), Ok(9));
    assert_eq!(tester.binary_search_by_key(&55, |&(_a, b)| b), Ok(12));
    assert_eq!(tester.binary_search_by_key(&-1, |&(_a, b)| b), Err(0));
    assert_eq!(tester.binary_search_by_key(&4, |&(_a, b)| b), Err(7));
    assert_eq!(tester.binary_search_by_key(&56, |&(_a, b)| b), Err(13));
    assert_eq!(tester.binary_search_by_key(&100, |&(_a, b)| b), Err(13));
}

// stdlib tests
#[test]
fn vd2_test_removex() {
    // This test checks that every single combination of tail position, length, and
    // removal position is tested. Capacity 15 should be large enough to cover every case.

    let mut tester = VecDeque2::with_capacity(16);
    // can't guarantee we got 15, so have to get what we got.
    // 15 would be great, but we will definitely get 2^k - 1, for k >= 4, or else
    // this test isn't covering what it wants to
    let cap = tester.capacity();

    // len is the length *after* removal
    let minlen = if cfg!(miri) { cap - 2 } else { 0 }; // Miri is too slow
    for len in minlen..cap - 1 {
        // 0, 1, 2, .., len - 1
        let expected = (0..).take(len).collect::<VecDeque2<_>>();
        for tail_pos in 0..cap {
            for to_remove in 0..=len {
                // Make the queue initially empty
                tester.tail = Counter(tail_pos);
                tester.head = Counter(tail_pos);
                for i in 0..len {
                    if i == to_remove {
                        tester.push_back(1234);
                    }
                    tester.push_back(i);
                }
                if to_remove == len {
                    tester.push_back(1234);
                }
                tester.remove(to_remove);
                assert!(tester.tail.0 < tester.cap() * 2);
                assert!(tester.head.0 < tester.cap() * 2);
                assert_eq!(tester, expected);
            }
        }
    }
}

#[test]
fn vd2_test_drain() {
    let mut tester: VecDeque2<usize> = VecDeque2::with_capacity(7);

    let cap = tester.capacity();
    for len in 0..=cap {
        for tail in 0..=cap {
            for drain_start in 0..=len {
                for drain_end in drain_start..=len {
                    tester.tail = Counter(tail);
                    tester.head = Counter(tail);
                    for i in 0..len {
                        tester.push_back(i);
                    }

                    // Check that we drain the correct values
                    let drained: VecDeque2<_> = tester.drain(drain_start..drain_end).collect();
                    let drained_expected: VecDeque2<_> = (drain_start..drain_end).collect();
                    assert_eq!(drained, drained_expected);

                    // We shouldn't have changed the capacity or made the
                    // head or tail out of bounds
                    assert_eq!(tester.capacity(), cap);
                    assert!(tester.tail.0 < tester.cap() * 2);
                    assert!(tester.head.0 < tester.cap() * 2);

                    // We should see the correct values in the VecDeque
                    let expected: VecDeque2<_> = (0..drain_start).chain(drain_end..len).collect();
                    assert_eq!(expected, tester);
                }
            }
        }
    }
}
