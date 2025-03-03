class Solution {
public:
    int numSubarrayBoundedMax(vector<int>& A, int L, int R) {
        vector<pair<int, int>> v;
        int n = 0, count = 0;
        for (int i = 0; i <= A.size(); i++) {
            if (i == A.size() || A[i] > R) {
                for (auto & p : v) {
                    count += (i - p.first) * p.second;
                }
                v.clear();
                n = 0;
            } else if (A[i] < L) {
                n++;
            } else {
                v.push_back(make_pair(i, n + 1));
                n = 0;
            }
        }
        return count;
    }
};