class Solution {
public:
    vector<int> majorityElement(vector<int>& nums) {
        int a = 0, b = 0, an = 0, bn = 0;
        for (auto & i : nums) {
            if (a == i) {
                an ++;
            } else if (b == i) {
                bn ++;
            } else if (an <= 0) {
                a = i;
                an = 1;
            } else if (bn <= 0) {
                b = i;
                bn = 1;
            } else {
                an --;
                bn --;
            }
        }
        an = bn = 0;
        for (auto & i : nums) {
            if (a == i) {
                an ++;
            } else if (b == i) {
                bn ++;
            }
        }
        vector<int> v;
        if (an > nums.size() / 3) {
            v.push_back(a);
        }
        if (bn > nums.size() / 3) {
            v.push_back(b);
        }
        return v;
    }
};