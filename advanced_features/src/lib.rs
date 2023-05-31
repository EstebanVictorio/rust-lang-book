#[cfg(test)]
mod tests {
    use super::split;

    #[test]
    fn it_splits_a_list_safely_from_an_unsafe_block() {
        let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let (left, right) = split(&mut values, 3);
        println!("Left: {:?}", left);
        println!("Right : {:?}", right);
        assert_eq!(left, &[1, 2, 3]);
    }
}

use std::slice;

fn split(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let pointer = values.as_mut_ptr();

    assert!(mid <= values.len());

    unsafe {
        (
            slice::from_raw_parts_mut(pointer, mid),
            slice::from_raw_parts_mut(pointer.add(mid), values.len() - mid),
        )
    }
}
