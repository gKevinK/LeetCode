// Sort
class Solution {
public:
    int findKthLargest(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());
        return nums[nums.size() - k];
    }
};

// Quick select
class Solution {
    int f(vector<int>& nums, int k, int l, int h) {
        if (h - l <= 1) return nums[k];
        int m = nums[h - 1];
        int i = l, j = h - 1;
        while (i < j) {
            while (i < j && nums[i] < m) i++;
            while (i < j && nums[j] >= m) j--;
            swap(nums[i], nums[j]);
        }
        swap(nums[i], nums[h - 1]);
        if (k == i) return nums[i];
        else if (k < i) return f(nums, k, l, i);
        else return f(nums, k, i + 1, h);
    }
public:
    int findKthLargest(vector<int>& nums, int k) {
        return f(nums, nums.size() - k, 0, nums.size());
    }
};