class Solution {
public:
    int minDominoRotations(vector<int>& A, vector<int>& B) {
        int n = A.size(), r = 1'000'000;
        for (int c : { A[0], B[0] }) {
            int n1 = 0, n2 = 0;
            for (int i = 0; i < n; i++) {
                if (A[i] != c && B[i] != c) {
                    n1 = n2 = -1'000'000;
                    break;
                }
                if (A[i] == c) n1++;
                if (B[i] == c) n2++;
            }
            r = min({ r, n - n1, n - n2 });
        }
        return r == 1'000'000 ? -1 : r;
    }
};