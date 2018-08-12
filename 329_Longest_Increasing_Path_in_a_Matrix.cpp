class Solution {
private:
    int setLength(vector<vector<int>>& matrix, vector<vector<int>>& v, int i, int j) {
        if (v[i][j] > 0) return v[i][j];
        int l[4] = { 0, 0, 0, 0 };
        if (j >= 1 && matrix[i][j] < matrix[i][j-1]) {
            l[0] = setLength(matrix, v, i, j-1);
        }
        if (i >= 1 && matrix[i][j] < matrix[i-1][j]) {
            l[1] = setLength(matrix, v, i-1, j);
        }
        if (j < matrix[0].size() - 1 && matrix[i][j] < matrix[i][j+1]) {
            l[2] = setLength(matrix, v, i, j+1);
        }
        if (i < matrix.size() - 1 && matrix[i][j] < matrix[i+1][j]) {
            l[3] = setLength(matrix, v, i+1, j);
        }
        l[0] = (l[0] > l[1]) ? l[0] : l[1];
        l[2] = (l[2] > l[3]) ? l[2] : l[3];
        l[0] = (l[0] > l[2]) ? l[0] : l[2];
        v[i][j] = l[0] + 1;
        return l[0] + 1;
    }

public:
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        if (matrix.size() == 0) return 0;
        vector<int> vt;
        vector<vector<int>> v;
        for (int i = 0; i < matrix[0].size(); i++) vt.push_back(0);
        for (int i = 0; i < matrix.size(); i++) v.push_back(vt);

        int max = 0, t;
        for (int i = 0; i < matrix.size(); i++) {
            for (int j = 0; j < matrix[0].size(); j++) {
                t = setLength(matrix, v, i, j);
                if (t > max) max = t;
            }
        }
        return max;
    }
};