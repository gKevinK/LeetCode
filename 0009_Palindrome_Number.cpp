class Solution {
public:
    bool isPalindrome(int x) {
        if (x < 0 || (x > 0 && x % 10 == 0)) return false;
        int t = 0;
        while (x > 0 && x != t) {
            t = t * 10 + x % 10;
            if (x == t) break;
            x /= 10;
        }
        return (x == t);
    }
};