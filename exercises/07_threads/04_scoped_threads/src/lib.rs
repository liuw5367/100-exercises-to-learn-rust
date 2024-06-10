// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return 0;
    }
    let mid = v.len() / 2;
    println!("mid: {mid}");

    let x = thread::scope(|s| {
        let a = s.spawn(|| {
            let arr: Vec<i32> = v[..mid].to_vec();
            let sum: i32 = arr.iter().sum();
            sum
        });

        let b = s.spawn(|| {
            let arr = v[mid..].to_vec();
            let sum: i32 = arr.iter().sum();
            sum
        });

        let aa = a.join().unwrap();
        let bb = b.join().unwrap();

        aa + bb
    });

    x
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
