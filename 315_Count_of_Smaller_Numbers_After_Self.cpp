class Solution {
    vector<int> f(vector<int>& nums, vector<int>& res, int lo, int hi) {
        if (hi - lo <= 1) return { lo };
        int mi = (hi - lo) / 2 + lo;
        vector<int> m1 = f(nums, res, lo, mi);
        vector<int> m2 = f(nums, res, mi, hi);
        vector<int> m(hi - lo);
        for (int i1 = 0, i2 = 0; i1 + i2 < hi - lo; ) {
            if (i2 == hi - mi || (i1 < mi - lo && nums[m1[i1]] <= nums[m2[i2]])) {
                res[m1[i1]] += i2;
                m[i1 + i2] = m1[i1];
                i1++;
            } else {
                m[i1 + i2] = m2[i2];
                i2++;
            }
        }
        return m;
    }
public:
    vector<int> countSmaller(vector<int>& nums) {
        vector<int> res(nums.size(), 0);
        f(nums, res, 0, nums.size());
        return res;
    }
};