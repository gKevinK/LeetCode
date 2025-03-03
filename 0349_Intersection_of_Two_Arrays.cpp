class Solution {
public:
    vector<int> intersection(vector<int>& nums1, vector<int>& nums2) {
        unordered_set<int> s(nums1.begin(), nums1.end());
        vector<int> r;
        for (int i : nums2) {
            if (s.find(i) != s.end()) {
                r.push_back(i);
                s.erase(i);
            }
        }
        return r;
    }
};