class Solution {
public:
    int reachNumber(int target) {
        target = abs(target);
        int n = sqrt(target * 2);
        while (n * (n + 1) / 2 < target) ++n;
        if ((n * (n + 1) / 2 - target) % 2 == 0) return n;
        else return n + 1 + (n % 2);
    }
};