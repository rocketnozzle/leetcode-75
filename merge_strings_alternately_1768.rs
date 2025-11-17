impl Solution {
    /// Merges two strings alternately, starting with characters from `word1`.
    ///
    /// If one string is longer than the other, the remaining characters of the
    /// longer string are appended to the end of the merged string.
    ///
    /// # Arguments
    ///
    /// * `word1` - The first string to merge.
    /// * `word2` - The second string to merge.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Solution::merge_alternately("abc".to_string(), "pqr".to_string());
    /// assert_eq!(s, "apbqcpr");
    ///
    /// let s = Solution::merge_alternately("ab".to_string(), "pqrs".to_string());
    /// assert_eq!(s, "apbqrs");
    /// ```
    ///
    /// # Intuition 💡
    ///
    /// The most direct way to merge the strings alternately is by using two index pointers (`i1` and `i2`).
    /// We iterate simultaneously through both strings, appending characters to the result.
    /// When one string is exhausted, we simply append the remaining tail of the other string.
    /// The provided implementation uses a `match` statement on the byte representation to handle all four
    /// possibilities (both exist, only one exists, neither exists) within a single loop.
    ///
    /// # Approach 📝
    ///
    /// 1. **Initialize:** Start with byte indices `i1 = 0` and `i2 = 0`.
    /// 2. **Pre-allocate:** Initialize the result string with pre-allocated capacity (`word1.len() + word2.len()`)
    ///    to optimize for speed by minimizing reallocations.
    /// 3. **Loop & Match:** Use an infinite `loop` that checks the current byte at `i1` and `i2`
    ///    using `match` on the result of `.as_bytes().get()`.
    ///    * **Case 1 (Both Exist):** Append character from `word1` then `word2`. Increment `i1` and `i2`.
    ///    * **Case 2 (Only `word1` Exists):** Append the remaining character and increment `i1`.
    ///    * **Case 3 (Only `word2` Exists):** Append the remaining character and increment `i2`.
    ///    * **Case 4 (Neither Exists):** Break the loop.
    /// 4. **Return:** Return the result `String`.
    ///
    /// *Note: This implementation assumes valid UTF-8 and uses byte iteration for simplicity, casting bytes to chars.*
    ///
    /// # Complexity 📈
    ///
    /// Let $N = \text{word1.len()}$ and $M = \text{word2.len()}$.
    ///
    /// * **Time complexity:** $O(N+M)$
    ///     We iterate through all $N+M$ characters exactly once.
    ///
    /// * **Space complexity:** $O(N+M)$
    ///     This is the space required to store the new merged string.
    pub fn merge_alternately(word1: String, word2: String) -> String {
        // Step 1: Create iterative positions for word1 and word2 (byte indices)
        let mut i1 = 0;
        let mut i2 = 0;
        // Step 2: Calculate the capacity for the result string
        // Setting capacity to the exact required length is sufficient.
        let mut result = String::with_capacity(word1.len() + word2.len());

        let bytes1 = word1.as_bytes();
        let bytes2 = word2.as_bytes();

        // Step 3: Get and append each character from word1 and word2
        loop {
            // Check the current byte at i1 and i2.
            match (bytes1.get(i1), bytes2.get(i2)) {
                (Some(&c1), Some(&c2)) => {
                    // Both exist: append c1 then c2, and advance both indices.
                    // Assuming valid ASCII/single-byte UTF-8 character for simplicity.
                    result.push(c1 as char);
                    result.push(c2 as char);
                    i1 += 1;
                    i2 += 1;
                }
                (Some(&c1), None) => {
                    // Only word1 exists: append c1 and advance i1.
                    result.push(c1 as char);
                    i1 += 1;
                }
                (None, Some(&c2)) => {
                    // Only word2 exists: append c2 and advance i2.
                    result.push(c2 as char);
                    i2 += 1;
                }
                (None, None) => {
                    // Neither exists: both strings are fully processed.
                    break;
                }
            }
        }
        
        result
    }
}
