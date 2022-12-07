#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub fn _3(s: &str) -> Option<usize> {
    if s.is_empty() {
        return None;
    }

    let mut start = 0;
    let mut ret = 0;

    let bytes = s.as_bytes();
    bytes.iter().enumerate().for_each(|(i, c)| {
        if let Some(duplicate) = bytes[start..i].iter().position(|v| v == c) {
            ret = ret.max(i - start);
            start += duplicate + 1;
        }
    });
    ret = ret.max(bytes.len() - start);

    Some(ret)
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
