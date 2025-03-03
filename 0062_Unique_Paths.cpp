class Solution {
public:
    int uniquePaths(int m, int n) {
        int *p = new int[m+1];
        memset(p, 0, (m+1) * sizeof(int));
        p[0] = 0;
        p[1] = 1;
        for (int i = 1; i < m + n - 1; i++) {
            for (int j = min(i+1, m); j >= max(1, i-n+2); j--) {
                p[j] = p[j] + p[j-1];
            }
        }
        return p[m];
    }
};