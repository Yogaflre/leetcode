// <最长连续序列>

// Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
// Your algorithm should run in O(n) complexity.

// Example:
// Input: [100, 4, 200, 1, 3, 2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

use std::cmp::max;
use std::collections::HashSet;
struct Solution;
impl Solution {
    /**
     * <Hash表> 天才解法
     * 用nums构建hashset，并遍历hashset
     * 当前元素是起始元素时(前一个元素不在set中)，不断遍历直到找不到下一个连续元素，并记录最大值
     */
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for n in nums {
            set.insert(n);
        }
        let mut res = 0;
        for start in &set {
            if !set.contains(&(start - 1)) {
                let mut end = *start + 1;
                while set.contains(&end) {
                    end += 1;
                }
                res = max(res, end - start);
            }
        }
        return res;
    }

    /**
     * 排序 + 遍历
     * 时间复杂度：O(nlogn)(不符合题意)
     */
    pub fn longest_consecutive2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut result = 1;
        let mut res = 1;
        let mut tmp = nums[0];
        for n in nums {
            if tmp + 1 == n {
                res += 1;
                result = max(res, result);
            } else if tmp != n {
                res = 1;
            }
            tmp = n;
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::longest_consecutive(vec![0]), 1);
    assert_eq!(
        Solution::longest_consecutive(vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]),
        7
    );
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
}
