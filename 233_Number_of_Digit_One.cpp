class Solution {
public:
    int tenExp(int x) {
        const int arr[10] = {1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000};
        return arr[x];
    }
    
    int countDigitOne(int n) {
        int res = n, l = 0, t1;
        int t[10] = {0};
        if (n <= 0) return 0;
        for (l = 0; l < 10 && res > 0; l++)
            res = res / 10;
        res = 0;
        t[1] = 1;
        for (int i = 2; i < l; i++)
            t[i] = t[i - 1] * 10 + tenExp(i - 1);
        for (int i = l - 1; i >= 0; i--) {
            t1 = n / tenExp(i) % 10;
            if (t1 == 1)
                res += n % tenExp(i) + 1;
            else if (t1 > 1)
                res += tenExp(i);
            res += t1 * t[i];
        }
        return res;
    }
};