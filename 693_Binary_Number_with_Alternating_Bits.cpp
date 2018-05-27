class Solution {
public:
    bool hasAlternatingBits(int n) {
        int d = 1 - n % 2;
        while (n) {
            if (d == n % 2)
                return false;
            d = n % 2;
            n /= 2;
        }
        return true;
    }
};