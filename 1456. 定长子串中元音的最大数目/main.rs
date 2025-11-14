impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let k = k as usize;
        let mut count = 0;
        let mut ans = 0;

        for i in 0..bytes.len() {
            if Self::is_vowel(bytes[i]) {
                count += 1;
            }
            // 原来的写法里 let left = i - k + 1 使用的是 usize，当 i < k - 1 时会产生下溢（无符号整数不能表示负数），结果变成一个非常大的数，比如错误里看到的 18446744073709551614。接下来访问 bytes[left] 时就越界了
            // let left = i - k + 1;
            if i + 1 < k {
                // 注意这里的判断不能出现负数 usize类型不能出现负数 不然就会越界
                continue;
            }
            let left = i - k + 1;
            ans = ans.max(count);

            if Self::is_vowel(bytes[left]) {
                count -= 1;
            }
        }

        ans
    }

    #[inline]
    fn is_vowel(b: u8) -> bool {
        matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
    }
}
// https://leetcode.cn/problems/maximum-number-of-vowels-in-a-substring-of-given-length/solutions/2809359/tao-lu-jiao-ni-jie-jue-ding-chang-hua-ch-fzfo/
