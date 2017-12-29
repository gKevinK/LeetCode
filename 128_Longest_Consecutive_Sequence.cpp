class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        unordered_map<int, int> m;
        int r = 0;
        for (int i : nums) {
            if (m[i]) continue;
            m[i] = m[i+m[i+1]] = m[i-m[i-1]] = m[i+m[i+1]] + m[i-m[i-1]] + 1;
            r = r > m[i] ? r : m[i];
        }
        return r;
    }
};