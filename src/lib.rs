pub mod error;
pub mod runtime_array;

#[cfg(test)]
mod test {
    use crate::runtime_array::Array;

    #[test]
    fn test_array_new() {
        let runt = Array::<i32>::new(10).unwrap();
        assert_ne!(true, runt.ptr().is_null());
    }

    #[test]
    fn test_array_index() {
        let mut runt = Array::<i32>::new(4).unwrap();
        println!("runt[0]: {}", runt[0]);
    }

    #[test]
    fn test_array_index_mut() {
        let mut runt = Array::<i32>::zeroed(9).unwrap();
        runt[4] = 4444;
        println!("runt[4] = {}", runt[4]);
        assert_eq!(runt[4], 4444);
    }

    #[test]
    fn test_array_into_iter() {
        let count = Array::<u64>::new(100);
    }
}
