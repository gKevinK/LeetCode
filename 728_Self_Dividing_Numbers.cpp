class Solution {
public:
    vector<int> selfDividingNumbers(int left, int right) {
        vector<int> v;
        for (int i = left; i<= right; i++) {
            int n = i;
            while (n) {
                int d = n % 10;
                if (d == 0 || i % d != 0)
                    break;
                n /= 10;
            }
            if (n == 0)
                v.push_back(i);
        }
        return v;
    }
};