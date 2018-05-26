class Solution {
public:
    int findShortestSubArray(vector<int>& nums) {
        unordered_map<int, tuple<int, int, int>> m;
        for (int i = 0; i < nums.size(); i++) {
            if (m.find(nums[i]) == m.end()) {
                m[nums[i]] = make_tuple(1, i, i);
            } else {
                get<0>(m[nums[i]])++;
                get<2>(m[nums[i]]) = i;
            }
        }
        int s1 = 0, s2 = 0;
        for (auto & p : m) {
            if (get<0>(p.second) > s1 || (get<0>(p.second) == s1 && get<2>(p.second) - get<1>(p.second) + 1 < s2)) {
                s1 = get<0>(p.second);
                s2 = get<2>(p.second) - get<1>(p.second) + 1;
            }
        }
        return s2;
    }
};