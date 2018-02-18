class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {
        for (int & n : nums) {
            int i = abs(n) - 1;
            if (nums[i] > 0)
                nums[i] = -nums[i];
        }
        vector<int> v;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] > 0)
                v.push_back(i + 1);
        }
        return v;
    }
};