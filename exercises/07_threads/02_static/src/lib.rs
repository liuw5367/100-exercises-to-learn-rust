// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(v: &'static [i32]) -> i32 {
    if v.is_empty() {
        return 0;
    }
    let mid = v.len() / 2;
    println!("mid: {mid}");

    let a = thread::spawn(move || {
        let arr = v[..mid].to_vec();
        let sum: i32 = arr.iter().sum();
        sum
    });

    let b = thread::spawn(move || {
        let arr = v[mid..].to_vec();
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
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
