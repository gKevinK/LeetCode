class Solution {
public:
    vector<double> medianSlidingWindow(vector<int>& nums, int k) {
        multiset<int> s = { nums[0] };
        decltype(s.begin()) t1 = s.begin(), t2 = s.begin();
        vector<double> r;
        if (k == 1) r.push_back(nums[0]);
        for (int i = 1; i < nums.size(); i++) {
            s.insert(nums[i]);
            if (t1 == t2) {
                if (*t2 <= nums[i]) t2++;
                else t1--;
            } else {
                if (*t1 <= nums[i]) t1++;
                if (nums[i] < *t2) t2--;
            }
            if (i >= k) {
                if (t1 == t2) {
                    if (nums[i - k] <= *t2) t2++;
                    s.erase(s.lower_bound(nums[i - k]));
                    t1 = t2;
                    t1--;
                } else {
                    if (nums[i - k] <= *t1) {
                        s.erase(s.lower_bound(nums[i - k]));
                        t1 = t2;
                    } else {
                        s.erase(s.lower_bound(nums[i - k]));
                        t2 = t1;
                    }
                }
            }
            if (i >= k - 1)
                r.push_back((static_cast<double>(*t1) + static_cast<double>(*t2)) / 2);
        }
        return r;
    }
};