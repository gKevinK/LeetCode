// class Solution {
//     int f(vector<int> & n, int l, int r) {
//         if (l + 1 == r) return l;
//         if (l + 2 == r) return (n[l] < n[r-1] ? l : r-1);
//         if (n[l] > n[(l+r)/2-1]) return f(n, l, (l+r)/2);
//         if (n[(l+r)/2] > n[r-1]) return f(n, (l+r)/2, r);
//         int a = f(n, l, (l+r)/2);
//         int b = f(n, (l+r)/2, r);
//         return (n[a] < n[b] ? a : b);
//     }
// public:
//     int findMin(vector<int>& nums) {
//         return nums[f(nums, 0, nums.size())];
//     }
// };

class Solution {
public:
    int findMin(vector<int>& nums) {
        int i = 0, j = nums.size() - 1;
        while (i < j) {
            int m = (i + j) / 2;
            if (nums[m] > nums[j])
                i = m + 1;
            else
                j = m;
        }
        return nums[i];
    }
};