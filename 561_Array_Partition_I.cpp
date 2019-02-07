class Solution {
public:
    int arrayPairSum(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        int r = 0, n = nums.size();
        for (int i = 0; i < n; i += 2)
            r += nums[i];
        return r;
    }
};

// class Solution {
// public:
//     int arrayPairSum(vector<int>& nums) {
//         vector<int> v(20001, 0);
//         for (int & i : nums)
//             v[i + 10000]++;
//         int r = 0, l = 0;
//         for (int i = 20000; i >= 0; i--)
//             if (v[i] > 0) {
//                 r += (v[i] + l) / 2 * (i - 10000);
//                 l = (v[i] + l) % 2;
//             }
//         return r;
//     }
// };