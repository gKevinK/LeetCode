class Solution {
    inline bool c(int a, int b) {
        return a / 2 > b || (a / 2 == b && a % 2 == 1);
    }
    
    vector<int> f(vector<int>& nums, int & r, int lo, int hi) {
        if (hi - lo == 1) return { nums[lo] };
        int mi = (hi - lo) / 2 + lo;
        vector<int> vr, v1 = f(nums, r, lo, mi), v2 = f(nums, r, mi, hi);
        for (int i = 0, j = 0; i < mi - lo && j < hi - mi; ) {
            if (c(v1[i], v2[j])) {
                r += mi - lo - i;
                j++;
            } else
                i++;
        }
        merge(v1.begin(), v1.end(), v2.begin(), v2.end(), back_inserter(vr));
        return vr;
    }
public:
    int reversePairs(vector<int>& nums) {
        int r = 0;
        if (!nums.empty())
            f(nums, r, 0, nums.size());
        return r;
    }
};