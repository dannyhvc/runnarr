/// # runtarr
///
/// This library uses the creation of runtime arrays in rust like in c++.
/// ```rs no_run
///
/// ```
pub mod runtime_array;

#[cfg(test)]
mod test {
    use crate::runtime_array::Array;

    #[test]
    fn test_array_new() {
        let runt = Array::<i32>::new(10);
        assert_ne!(true, runt.ptr().is_null());
    }

    #[test]
    fn test_array_index() {
        let runt = Array::<i32>::new(4);
        println!("runt[0]: {:?}", runt[0]);
        assert_ne!(runt[0], 0);
    }

    #[test]
    fn test_array_index_mut() {
        let mut runt = Array::<i32>::new(9);
        runt[4] = 4444;
        println!("runt[4] = {:?}", runt[4]);
        assert_eq!(runt[4], 4444);
    }

    #[test]
    fn test_array_into_iter() {
        let runt = Array::<u64>::new(100).into_iter().count();
        println!("{}", runt);
    }
}
