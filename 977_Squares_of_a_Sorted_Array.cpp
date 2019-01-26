class Solution {
public:
    vector<int> sortedSquares(vector<int>& A) {
        int r = distance(A.begin(), lower_bound(A.begin(), A.end(), 0));
        int l = r - 1, n = A.size();
        vector<int> res;
        while (l >= 0 || r < n) {
            if (l < 0 || (r < n && abs(A[l]) > abs(A[r]))) {
                res.push_back(A[r] * A[r]);
                r++;
            } else {
                res.push_back(A[l] * A[l]);
                l--;
            }
        }
        return res;
    }
};