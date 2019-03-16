class Solution {
public:
    int longestOnes(vector<int>& A, int K) {
        int i = 0, j = 0, r = 0, n = A.size();
        for (; i < n && K; i++) {
            if (A[i] == 0) K--;
            r = max(r, i - j + 1);
        }
        for (; i < n; i++) {
            if (A[i] == 0) {
                while (A[j] == 1) j++;
                j++;
            }
            r = max(r, i - j + 1);
        }
        return r;
    }
};