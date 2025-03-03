class Solution {
public:
    int findPairs(vector<int>& nums, int k) {
        if (nums.empty() || k < 0) return 0;
        unordered_map<int, int> m;
        int count = 0;
        for (int & i : nums)
            m[i]++;
        if (k == 0) {
            for (auto p : m)
                if (p.second >= 2)
                    count++;
        } else if (k > 0) {
            for (auto p : m)
                if (m.find(p.first + k) != m.end())
                    count++;
        }
        return count;
    }
};