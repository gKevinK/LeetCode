class Solution {
public:
    string optimalDivision(vector<int>& nums) {
        string s = to_string(nums[0]);
        if (nums.size() == 1)
            return s;
        else if (nums.size() == 2)
            return s + "/" + to_string(nums[1]);
        else {
            s += "/(";
            for (int i = 1; i < nums.size(); i++)
                s += to_string(nums[i]) + "/";
            s.back() = ')';
            return s;
        }
    }
};