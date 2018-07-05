class Solution {
public:
    bool containsNearbyAlmostDuplicate(vector<int>& nums, int k, int t) {
        if (t < 0) return false;
        map<int, int> m;
        int n = nums.size();
        for (int i = 0; i < n; i++) {
            int c = nums[i];
            if (m.lower_bound(INT_MIN + t > c ? INT_MIN : c - t) != m.upper_bound(INT_MAX - t < c ? INT_MAX : c + t)) return true;
            m[nums[i]]++;
            if (i >= k) {
                m[nums[i - k]]--;
                if (m[nums[i - k]] == 0)
                    m.erase(nums[i - k]);
            }
        }
        return false;
    }
};