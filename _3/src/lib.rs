#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub fn _3(s: &str) -> Option<usize> {
    (0..s.len())
        .map(|i| get_longest_unique(&s.as_bytes()[i..]))
        .max()
        .flatten()
}

fn get_longest_unique(buf: &[u8]) -> Option<usize> {
    buf.iter()
        .enumerate()
        .position(|(i, v)| buf[..i].contains(v))
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
