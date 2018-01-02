class Solution {
public:
    void sortColors(vector<int>& nums) {
        int colors[] = { 0, 0, 0 };
        for (int i : nums) {
            colors[i] ++;
        }
        for (int i = 0; i < colors[0]; i++) nums[i] = 0;
        for (int i = colors[0]; i < colors[0] + colors[1]; i++) nums[i] = 1;
        for (int i = colors[0] + colors[1]; i < nums.size(); i++) nums[i] = 2;
    }
};