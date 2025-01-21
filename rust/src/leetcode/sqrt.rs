// 69. Sqrt(x) https://leetcode.com/problems/sqrtx/

pub mod sqrtsolution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x
        }

        let mut l: i32 = 2;
        let mut r: i32 = x / 2;

        while l <= r {
            let mid: i32 = l + (r - l) / 2;
            let c: i64 = mid as i64 * mid as i64;

            if c < x as i64 {
                l = mid + 1;
            } else if c > x as i64 {
                r = mid - 1;
            } else {
               return mid;
            }
        }
        r
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::sqrtsolution::my_sqrt;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(16), 4);
        assert_eq!(my_sqrt(2147395599), 46339);
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(1000000), 1000);
    }
}