// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return 0;
    }
    let v: &'static mut [i32] = Box::leak(v.into_boxed_slice());
    let mid = v.len() / 2;
    println!("mid: {mid}");

    let arr = v[..mid].to_vec();
    let a = thread::spawn(move || {
        let sum: i32 = arr.iter().sum();
        sum
    });

    let arr = v[mid..].to_vec();
    let b = thread::spawn(move || {
        let sum: i32 = arr.iter().sum();
        sum
    });

    let aa = a.join().unwrap();
    let bb = b.join().unwrap();

    aa + bb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
