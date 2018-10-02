class Solution {
public:
    int smallestRangeI(vector<int>& A, int K) {
        int lo = *min_element(A.begin(), A.end());
        int hi = *max_element(A.begin(), A.end());
        return max(0, hi - lo - 2 * K);
    }
};