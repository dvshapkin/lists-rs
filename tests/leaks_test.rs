use lists::List;

#[test]
fn leaks_check() {
    let mut list = List::new();
    list.push_back(vec![1u8, 2, 3]);
    list.push_back(vec![1, 4, 3]);
    list.push_back(vec![1, 5, 3]);

    let item_0 = list.pop_front();
    let item_2 = list.pop_back();

    assert_eq!(list.len(), 1);
}
