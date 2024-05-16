use crate::runtime_array::ArrayCStyle;

#[test]
fn test_array_new() {
    let runt = ArrayCStyle::<i32>::new(10).unwrap();
    assert_ne!(true, runt.ptr().is_null());
}

#[test]
fn test_array_index() {
    let _runt = ArrayCStyle::<i32>::new(4).unwrap();
    println!("runt[0]: {}", _runt[0]);
}

#[test]
fn test_array_index_mut() {
    let mut runt = ArrayCStyle::<i32>::zeroed(9).unwrap();
    runt[4] = 4444;
    println!("runt[4] = {}", runt[4]);
    assert_eq!(runt[4], 4444);
}

#[test]
fn test_array_into_iter() {
    let count = ArrayCStyle::<u64>::new(100);
    let mut iter = count.iter();
    iter.next();
}

#[test]
fn bs() {
    // TODO
    type X<T: Default> = ArrayCStyle<T>;
    let x = X::<i32>::new(0);
}
