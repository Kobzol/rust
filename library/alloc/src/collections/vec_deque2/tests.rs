use super::*;

#[test]
fn vd2_test_get() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    assert_eq!(tester.len(), 0);
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
fn vd2_test_pop_back() {
    let mut tester: VecDeque2<u64> = VecDeque2::new();
    assert_eq!(tester.len(), 0);
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);
    tester.pop_back();

    assert_eq!(tester.len(), 2);

    assert_eq!(tester.get(0), Some(&1));
    assert_eq!(tester.get(1), Some(&2));

    tester.pop_back();
    tester.pop_back();
    assert_eq!(tester.len(), 0);
}

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
fn vd2_test_iter() {
    let mut tester = VecDeque2::new();
    tester.push_back(1);
    tester.push_back(2);
    tester.push_back(3);

    for (index, &item) in tester.iter().enumerate() {
        assert_eq!(item, index + 1);
    }
}
