class Solution {
public:
    int countPrimes(int n) {
        vector<bool> v(n, false);
        int sum = 0;
        for (int i = 2; i < n; i++) {
            if (!v[i]) {
                sum++;
                for (int j = i * 2; j < n; j += i) {
                    v[j] = true;
                }
            }
        }
        return sum;
    }
};