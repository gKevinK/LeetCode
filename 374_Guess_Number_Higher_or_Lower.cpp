// Forward declaration of guess API.
// @param num, your guess
// @return -1 if my number is lower, 1 if my number is higher, otherwise return 0
int guess(int num);

class Solution {
public:
    int guessNumber(int n) {
        int l = 1, h = n, m = 1;
        while (l <= h) {
            m = (h - l) / 2 + l;
            int r = guess(m);
            if (r == 1) l = m + 1;
            else if (r == -1) h = m - 1;
            else return m;
        }
        return l;
    }
};