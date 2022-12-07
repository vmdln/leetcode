#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub fn _1(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    nums.iter().enumerate().find_map(|(i, x)| {
        nums[i + 1..]
            .iter()
            .position(|y| x + y == target)
            .map(|j| (i, i + 1 + j))
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn _1() {
        let nums = [2, 7, 11, 15];
        let target = 9;

        let ret = Some((0, 1));

        assert_eq!(crate::_1(&nums, target), ret);
    }

    #[test]
    fn _2() {
        let nums = [3, 2, 4];
        let target = 6;

        let ret = Some((1, 2));

        assert_eq!(crate::_1(&nums, target), ret);
    }

    #[test]
    fn _3() {
        let nums = [3, 3];
        let target = 6;

        let ret = Some((0, 1));

        assert_eq!(crate::_1(&nums, target), ret);
    }
}
