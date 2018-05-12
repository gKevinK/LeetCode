class Solution {
public:
    bool xorGame(vector<int>& nums) {
        int s = 0;
        for (int & i : nums)
            s ^= i;
        return s == 0 || nums.size() % 2 == 0;
    }
};