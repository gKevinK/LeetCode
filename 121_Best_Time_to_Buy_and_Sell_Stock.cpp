class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int buy = 0x7FFFFFFF, pro = 0;
        for (auto i : prices) {
            if (i < buy) {
                buy = i;
            }
            if (i - buy > pro) {
                pro = i - buy;
            }
        }
        return pro;
    }
};