// Forward declaration of isBadVersion API.
bool isBadVersion(int version);

class Solution {
public:
    int firstBadVersion(int n) {
        int l = 0, h = n;
        while (l < h) {
            int m = (h - l) / 2 + l;
            if (isBadVersion(m)) {
                h = m;
            } else {
                l = m + 1;
            }
        }
        return l;
    }
};