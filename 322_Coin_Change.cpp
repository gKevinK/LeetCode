// Slow
class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        unordered_set<int> s1 = { 0 }, s2;
        int n = 0;
        if (amount == 0)
            return 0;
        while (!s1.empty()) {
            for (const int & i : s1) {
                for (const int & c : coins) {
                    if (i + c == amount)
                        return n + 1;
                    if (i + c < amount)
                        s2.insert(i + c);
                }
            }
            n++;
            s1.clear();
            s1.swap(s2);
        }
        return -1;
    }
};

// DP
class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        vector<int> v(amount + 1, amount + 1);
        v[0] = 0;
        for (int & c : coins) {
            for (int i = 1; i <= amount; i++) {
                if (i - c >= 0)
                    v[i] = min(v[i], v[i - c] + 1);
            }
        }
        return v.back() == amount + 1 ? -1 : v.back();
    }
};