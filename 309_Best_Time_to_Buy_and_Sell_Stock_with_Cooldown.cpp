class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int res = 0, hold = INT_MIN, sold = 0, sold2 = 0;
        for (int & price : prices) {
            int tsold = price + hold;
            int thold = max(hold, sold2 - price);
            sold2 = max(sold, sold2);
            sold = tsold;
            hold = thold;
            res = max(res, sold);
        }
        return res;
    }
};