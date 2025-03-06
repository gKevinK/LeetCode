class Solution {
public:
    int numberOfArithmeticSlices(vector<int>& A) {
        int r = 0;
        for (int i = 0, j = 0; i + 2 < A.size(); i = j) {
            int m = A[i+1] - A[i];
            for (j = i + 1; j + 1 < A.size() && A[j+1] - A[j] == m; j++);
            for (int k = 3; k <= j - i + 1; k++)
                r += j - i + 2 - k;
        }
        return r;
    }
};