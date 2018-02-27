class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int *p = new int[prices.size()];
        int buy = 0x7FFFFFFF, sell = 0, pro = 0;
        for (int i = 0; i < prices.size(); i++) {
            if (prices[i] < buy) {
                buy = prices[i];
            }
            if (prices[i] - buy > pro) {
                pro = prices[i] - buy;
            }
            p[i] = pro;
        }
        pro = 0;
        for (int i = prices.size() - 1; i >= 0; i--) {
            if (prices[i] > sell) {
                sell = prices[i];
            }
            if (sell - prices[i] > pro) {
                pro = sell - prices[i];
            }
            p[i] += pro;
        }
        int max = 0;
        for (int i = 0; i < prices.size(); i++) {
            if (p[i] > max) max = p[i];
        }
        return max;
    }
};