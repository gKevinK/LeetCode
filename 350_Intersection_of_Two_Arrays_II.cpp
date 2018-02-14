class Solution {
public:
    vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
        if (nums1.size() < nums2.size()) return intersect(nums2, nums1);
        vector<int> v;
        vector<int> f(nums2.size(), 0);
        for (int i : nums1) {
            for (int j = 0; j < nums2.size(); j++) {
                if (i == nums2[j] && f[j] == 0) {
                    v.push_back(i);
                    f[j] = 1;
                    break;
                }
            }
        }
        return v;
    }
};