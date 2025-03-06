class Solution {
public:
    vector<int> singleNumber(vector<int>& nums) {
        int d = 0;
        for (int i : nums) {
            d ^= i;
        }
        d &= -d;
        int a = 0, b = 0;
        for (int i : nums) {
            if ((i & d) == 0)
                a ^= i;
            else 
                b ^= i;
        }
        return vector<int> {a, b};
    }
};