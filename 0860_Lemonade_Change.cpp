class Solution {
public:
    bool lemonadeChange(vector<int>& bills) {
        int five = 0, ten = 0;
        for (int b : bills) {
            if (b == 5)
                five++;
            else if (b == 10) {
                ten++;
                five--;
                if (five < 0) return false;
            } else {
                if (ten) {
                    ten--;
                    five--;
                } else {
                    five -= 3;
                }
                if (five < 0 || ten < 0) return false;
            }
        }
        return true;
    }
};