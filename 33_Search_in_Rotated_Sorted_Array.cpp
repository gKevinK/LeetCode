class Solution {
private:
    int s(vector<int>& nums, int target, int l, int r) {
        if (r - l == 0) return -1;
        if (r - l == 1 && nums[l] != target) return -1;
        if (nums[l] == target) return l;
        int m = (l + r) / 2;
        int a = -1;
        if (!(nums[l] > nums[m-1] && (nums[l] < target && target < nums[m-1])))
            a = s(nums, target, l, m);
        if (a == -1 && !(nums[m] > nums[r-1] && (nums[m] < target && target < nums[r-1])))
            a = s(nums, target, m, r);
        return a;
    }
    
public:
    int search(vector<int>& nums, int target) {
        if (nums.size() == 0) return -1;
        return s(nums, target, 0, nums.size());
    }
};