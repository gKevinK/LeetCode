class Solution {
public:
    vector<int> sumEvenAfterQueries(vector<int>& A, vector<vector<int>>& queries) {
        int s = 0;
        vector<int> res;
        for (int & i : A)
            if (i % 2 == 0)
                s += i;
        for (auto & q : queries) {
            if (A[q[1]] % 2 == 0)
                s -= A[q[1]];
            A[q[1]] += q[0];
            if (A[q[1]] % 2 == 0)
                s += A[q[1]];
            res.push_back(s);
        }
        return res;
    }
};