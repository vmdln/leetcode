#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use itertools::Itertools;
use std::collections::LinkedList;

pub fn _2(l1: &LinkedList<u8>, l2: &LinkedList<u8>) -> LinkedList<u8> {
    let mut ret = LinkedList::new();
    let mut carry = 0;

    l1.iter()
        .zip_longest(l2.iter())
        .map(|ab| ab.or(&0, &0))
        .for_each(|(a, b)| {
            let (v, c) = add_carry(*a, *b, carry);

            carry = c;
            ret.push_back(v);
        });

    if carry != 0 {
        ret.push_back(carry);
    }

    ret
}

fn add_carry(a: u8, b: u8, carry: u8) -> (u8, u8) {
    let tmp = a + b + carry;

    let v = tmp % 10;
    let c = tmp / 10;

    (v, c)
}

#[cfg(test)]
mod tests {
    #[test]
    fn _1() {
        let a = [2, 4, 3].into_iter().collect();
        let b = [5, 6, 4].into_iter().collect();

        let ans = [7, 0, 8].into_iter().collect();

        assert_eq!(crate::_2(&a, &b), ans);
    }

    #[test]
    fn _2() {
        let a = [0].into_iter().collect();
        let b = [0].into_iter().collect();

        let ans = [0].into_iter().collect();

        assert_eq!(crate::_2(&a, &b), ans);
    }

    #[test]
    fn _3() {
        let a = [9, 9, 9, 9, 9, 9, 9].into_iter().collect();
        let b = [9, 9, 9, 9].into_iter().collect();

        let ans = [8, 9, 9, 9, 0, 0, 0, 1].into_iter().collect();

        assert_eq!(crate::_2(&a, &b), ans);
    }
}
