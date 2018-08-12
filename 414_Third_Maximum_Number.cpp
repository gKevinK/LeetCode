class Solution {
public:
    int thirdMax(vector<int>& nums) {
        int a = INT_MIN, b = INT_MIN, c = INT_MIN;
        bool af = false, bf = false, cf = false;
        if (nums[0] == INT_MIN) af = true;
        for (auto & i : nums) {
            if (i > c && i != a && i != b || cf == false) {
                if (i > b) {
                    c = b; cf = bf;
                    if (i > a) {
                        b = a; bf = af;
                        a = i; af = true;
                    } else {
                        b = i; bf = true;
                    }
                } else {
                    c = i; cf = true;
                }
            }
        }
        if (cf) return c;
        else return a;
    }
};