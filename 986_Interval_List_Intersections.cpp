class Solution {
public:
    vector<vector<int>> intervalIntersection(vector<vector<int>>& A, vector<vector<int>>& B) {
        int i = 0, j = 0;
        vector<vector<int>> r;
        while (i < A.size() && j < B.size()) {
            if (A[i][0] < B[j][0]) {
                if (A[i][1] < B[j][0]) {
                    i++;
                } else if (A[i][1] < B[j][1]) {
                    r.push_back({ B[j][0], A[i][1] });
                    ++i;
                } else {
                    r.push_back({ B[j][0], B[j][1] });
                    ++j;
                }
            } else {
                if (A[i][0] > B[j][1]) {
                    j++;
                } else if (A[i][1] < B[j][1]) {
                    r.push_back({ A[i][0], A[i][1] });
                    i++;
                } else {
                    r.push_back({ A[i][0], B[j][1] });
                    j++;
                }
            }
        }
        return r;
    }
};