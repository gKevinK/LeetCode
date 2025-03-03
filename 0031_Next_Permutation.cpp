class Solution {
public:
    void nextPermutation(vector<int>& nums) {
        int n = nums.size();
        int t = n - 1;
        while (t > 0 && nums[t - 1] >= nums[t])
            t--;
        if (t == 0) {
            reverse(nums.begin(), nums.end());
        } else {
            int p = t;
            while (p + 1 < n && nums[p + 1] > nums[t - 1])
                p++;
            swap(nums[t - 1], nums[p]);
            for (int i = t, j = n - 1; i < j; i++, j--)
                swap(nums[i], nums[j]);
        }
    }
};

// class Solution {
// public:
//     void nextPermutation(vector<int>& nums) {
//         next_permutation(nums.begin(), nums.end());
//     }
// };