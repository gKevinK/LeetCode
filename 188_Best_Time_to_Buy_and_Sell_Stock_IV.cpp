class Solution {
private:
    int maxProfit2(vector<int>& prices) {
        int pro = 0;
        for (int i = 1; i < prices.size(); i++) {
            if (prices[i] > prices[i-1]) {
                pro += prices[i] - prices[i-1];
            }
        }
        return pro;
    }

public:
    int maxProfit(int k, vector<int>& prices) {
        if (prices.empty()) return 0;
        if (k >= prices.size()) return maxProfit2(prices);
        int l[k + 1] = { 0 };
        int g[k + 1] = { 0 };
        for (int i = 1; i < prices.size(); ++i) {
            int diff = prices[i] - prices[i - 1];
            for (int j = k; j > 0; --j) {
                l[j] = max(l[j] + diff, g[j - 1] + max(diff, 0));
                g[j] = max(l[j], g[j]);
            }
        }
        return g[k];
    }
};