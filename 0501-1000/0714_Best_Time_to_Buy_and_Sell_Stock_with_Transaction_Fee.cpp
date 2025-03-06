class Solution {
public:
    int maxProfit(vector<int>& prices, int fee) {
        int d1 = 0, d2 = INT_MIN, td1;
        for (int i = 0; i < prices.size(); i++) {
            td1 = d1;
            d1 = max(d1, d2 + prices[i]);
            d2 = max(d2, td1 - prices[i] - fee);
        }
        return d1;
    }
};