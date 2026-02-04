class Solution {
public:
    int maxProduct(vector<int>& nums, int k, int limit) {
        const static int MIN_PROD[41] = {1,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 24, 36, 48, 60, 72, 84, 96, 108,
            120, 132, 144, 288, 432, 576, 720, 864, 1008, 1152,
            1296, 1440, 1584, 1728, 3456, 5184, 6912, 8640, 10368, 12096};

        int res = -1;
        unordered_set<int> s;
        s.insert(15000);
        vector<int> qadd;
        // Try subsequences without zero
        for (int n : nums) {
            if (n == 0)
                continue;
            for (auto & t : s) {
                int sign = (t > 0) ? 1 : -1;
                int asum = (t * sign) % 10000 - 5000;
                int prod = (t * sign) / 10000;
                int asum2 = asum + sign * n;
                int sign2 = -sign;
                int prod2 = prod * n;
                if (asum2 == k && prod2 <= limit) {
                    res = max(res, prod2);
                }
                int rest = abs(k - asum2) + ((sign2 * (k - asum2) >= 0) ? 0 : 1);
                int rest_prod = MIN_PROD[min(rest, 40)];
                if (prod2 * rest_prod <= limit) {
                    qadd.push_back(sign2 * (prod2 * 10000 + asum2 + 5000));
                }
            }
            for (auto & t : qadd) {
                s.insert(t);
            }
            qadd.clear();
        }
        // Try subsequences with zero
        if (res == -1) {
            s.clear();
            s.insert(15000);
            for (int n : nums) {
                for (auto & t : s) {
                    int sign = t > 0 ? 1 : -1;
                    int asum = ((t * sign) % 10000) - 5000;
                    int prod = (t * sign) / 10000;
                    int asum2 = asum + sign * n;
                    int sign2 = -sign;
                    int prod2 = prod * (n > 0 ? 1 : 0);
                    if (asum2 == k && prod2 == 0) {
                        return 0;
                    }
                    qadd.push_back(sign2 * (prod2 * 10000 + asum2 + 5000));
                }
                for (auto & t : qadd) {
                    s.insert(t);
                }
                qadd.clear();
            }
        }
        return res;
    }
};