class Solution {
public:
    int lenLongestFibSubseq(vector<int>& A) {
        unordered_set<int> s(A.begin(), A.end());
        int n = A.size(), mlen = 0;
        for (int i = 0; i < n - 2; i++) {
            for (int j = i + 1; j < n - 1; j++) {
                int len = 2, a = A[i], b = A[j];
                if (s.find(a + b) == s.end()) continue;
                while (s.find(a + b) != s.end()) {
                    b = a + b;
                    a = b - a;
                    len++;
                }
                mlen = max(mlen, len);
            }
        }
        return mlen;
    }
};