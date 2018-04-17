class Solution {
public:
    int minCostClimbingStairs(vector<int>& cost) {
        int c1 = 0, c2 = 0;
        for (int i = 2; i <= cost.size(); i++) {
            int t = min(c1 + cost[i - 2], c2 + cost[i - 1]);
            c1 = c2;
            c2 = t;
        }
        return c2;
    }
};