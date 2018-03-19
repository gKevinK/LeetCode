class Solution {
private:
    void sub(vector<vector<int>>& matrix, int curr, int r) {
        int s1 = matrix.size();
        if (s1 - 2 * r == 0) return;
        int s2 = matrix[0].size();
        if (s2 - 2 * r == 0) return;
        else if (s1 - 2 * r == 1) {
            for (int i = r; i <= s2 - r - 1; ++i) {
                matrix[r][i] = curr++;
            }
        } else if (s2 - 2 * r == 1) {
            for (int i = r; i <= s1 - r - 1; ++i) {
                matrix[i][r] = curr++;
            }
        } else {
            for (int i = r; i < s2 - r - 1; ++i) {
                matrix[r][i] = curr++;
            }
            for (int i = r; i < s1 - r - 1; ++i) {
                matrix[i][s2 - r - 1] = curr++;
            }
            for (int i = s2 - r - 1; i > r; --i) {
                matrix[s1 - r - 1][i] = curr++;
            }
            for (int i = s1 - r - 1; i > r; --i) {
                matrix[i][r] = curr++;
            }
            sub(matrix, curr, r + 1);
        }
    }

public:
    vector<vector<int>> generateMatrix(int n) {
        vector<int> v;
        v.resize(n);
        vector<vector<int>> matrix;
        for (int i = 0; i < n; ++i) {
            matrix.push_back(v);
        }
        sub(matrix, 1, 0);
        return matrix;
    }
};