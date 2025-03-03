class Solution {
public:
    int findLHS(vector<int>& nums) {
        unordered_map<int, int> m;
        for (const int & i : nums)
            m[i]++;
        int r = 0;
        for (auto & p : m)
            if (m.find(p.first + 1) != m.end())
                r = max(r, p.second + m[p.first + 1]);
        return r;
    }
};