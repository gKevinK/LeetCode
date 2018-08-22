class Solution {
public:
    int maxRotateFunction(vector<int>& A) {
        int sum = 0, f = 0, n = A.size();
        for (int & i : A)
            sum += i;
        for (int i = 0; i < n; i++)
            f += i * A[i];
        int res = f;
        for (int i = n - 1; i > 0; i--) {
            f += sum - n * A[i];
            res = max(res, f);
        }
        return res;
    }
};