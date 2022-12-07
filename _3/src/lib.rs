#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::bool_comparison)]

pub fn _3(s: &str) -> Option<usize> {
    let mut start = 0;

    let bytes = s.as_bytes();
    let ret = bytes
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            bytes[start..i]
                .iter()
                .position(|v| v == c)
                .map(|duplicate| {
                    let tmp = i - start;
                    start += duplicate + 1;

                    tmp
                })
        })
        .max();

    match ret {
        Some(v) => Some(v.max(bytes.len() - start)),
        None if bytes.is_empty() == false => Some(bytes.len()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _1() {
        let x = "abcabcbb";
        let ans = Some(3);

        assert_eq!(crate::_3(x), ans);
    }

    #[test]
    fn _2() {
        let x = "bbbbb";
        let ans = Some(1);

        assert_eq!(crate::_3(x), ans);
    }

    #[test]
    fn _3() {
        let x = "pwwkew";
        let ans = Some(3);

        assert_eq!(crate::_3(x), ans);
    }
}
