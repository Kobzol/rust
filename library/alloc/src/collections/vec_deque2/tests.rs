use super::*;
use crate::string::ToString;

#[test]
fn vd2_test_empty_1() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    assert_eq!(tester.len(), 0);
    assert!(tester.is_empty());
    tester.push_back(1);
    assert_eq!(tester.len(), 1);
    assert!(!tester.is_empty());
}

#[test]
fn vd2_test_empty_2() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    assert_eq!(tester.len(), 0);
    assert!(tester.is_empty());
    tester.push_front(1);
    assert_eq!(tester.len(), 1);
    assert!(!tester.is_empty());
}

#[test]
fn vd2_test_get() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    assert_eq!(tester.len(), 3);

    assert_eq!(tester.get(0), Some(&1));
    assert_eq!(tester.get(1), Some(&2));
    assert_eq!(tester.get(2), Some(&3));
    assert_eq!(tester.get(3), None);
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

/*
def test_grow_t_h_in_bounds():
    vd = VecDeque.from_str("t1,2,3,h|_,_,_,_")
    check_grow(vd, [1, 2, 3])
    assert vd.to_str() == "t1,2,3,h,_,_,_,_|_,_,_,_,_,_,_,_"


def test_grow_t_h_overflow():
    vd = VecDeque.from_str("1,_,_,_|t,h,_,_")
    check_grow(vd, [1])
    assert vd.to_str() == "t1,h,_,_,_,_,_,_|_,_,_,_,_,_,_,_"


def test_grow_h_t_nothing_to_copy():
    vd = VecDeque.from_str("_,t1,2,3|h,_,_,_")
    check_grow(vd, [1, 2, 3])
    assert vd.to_str() == "_,t1,2,3,h,_,_,_|_,_,_,_,_,_,_,_"


def test_grow_h_t_copy_after_t():
    vd = VecDeque.from_str("3,_,t1,2|_,h,_,_")
    check_grow(vd, [1, 2, 3])
    assert vd.to_str() == "_,_,t1,2,3,h,_,_|_,_,_,_,_,_,_,_"


def test_grow_h_t_copy_move_t():
    vd = VecDeque.from_str("2,3,_,t1|_,_,h,_")
    check_grow(vd, [1, 2, 3])
    assert vd.to_str() == "2,3,h,_,_,_,_,1|_,_,_,_,_,_,_,t"


def test_grow_full_beginning_1():
    vd = VecDeque.from_str("t1,2,3,4|h,_,_,_")
    check_grow(vd, [1, 2, 3, 4])
    assert vd.to_str() == "t1,2,3,4,h,_,_,_|_,_,_,_,_,_,_,_"


def test_grow_full_beginning_2():
    vd = VecDeque.from_str("h1,2,3,4|t,_,_,_")
    check_grow(vd, [1, 2, 3, 4])
    assert vd.to_str() == "t1,2,3,4,h,_,_,_|_,_,_,_,_,_,_,_"


def test_grow_full_end_1():
    vd = VecDeque.from_str("1,2,3,t4|_,_,_,h")
    check_grow(vd, [4, 1, 2, 3])
    assert vd.to_str() == "1,2,3,h,_,_,_,4|_,_,_,_,_,_,_,t"


def test_grow_full_end_2():
    vd = VecDeque.from_str("2,3,4,h1|_,_,_,t")
    check_grow(vd, [1, 2, 3, 4])
    assert vd.to_str() == "2,3,4,h,_,_,_,1|_,_,_,_,_,_,_,t"


def test_grow_full_middle_copy_after_t_1():
    vd = VecDeque.from_str("4,h1,2,3|_,t,_,_")
    check_grow(vd, [1, 2, 3, 4])
    assert vd.to_str() == "_,t1,2,3,4,h,_,_|_,_,_,_,_,_,_,_"


def test_grow_full_middle_copy_after_t_2():
    vd = VecDeque.from_str("4,t1,2,3|_,h,_,_")
    check_grow(vd, [1, 2, 3, 4])
    assert vd.to_str() == "_,t1,2,3,4,h,_,_|_,_,_,_,_,_,_,_"


def test_grow_full_middle_move_t_1():
    vd = VecDeque.from_str("4,5,6,7,8,t1,2,3|_,_,_,_,_,h,_,_")
    check_grow(vd, [1, 2, 3, 4, 5, 6, 7, 8])
    assert vd.to_str() == "4,5,6,7,8,h,_,_,_,_,_,_,_,1,2,3|_,_,_,_,_,_,_,_,_,_,_,_,_,t,_,_"


def test_grow_full_middle_move_t_2():
    vd = VecDeque.from_str("4,5,6,7,8,h1,2,3|_,_,_,_,_,t,_,_")
    check_grow(vd, [1, 2, 3, 4, 5, 6, 7, 8])
    assert vd.to_str() == "4,5,6,7,8,h,_,_,_,_,_,_,_,1,2,3|_,_,_,_,_,_,_,_,_,_,_,_,_,t,_,_"

*/

/*
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
*/

/*#[test]
fn vd2_test_contains() {
    let mut tester = VecDeque2::new();
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
*/

/*#[test]
fn vd2_test_clear() {
    let mut tester = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);
    tester.clear();
    assert_eq!(tester.len(), 0);
}

#[test]
fn vd2_test_iter() {
    let mut tester = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    for (index, &item) in tester.iter().enumerate() {
        assert_eq!(item, index + 1);
    }
}
*/
