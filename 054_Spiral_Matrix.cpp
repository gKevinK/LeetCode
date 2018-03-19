class Solution {
private:
    void sub(vector<vector<int>>& matrix, vector<int>& seq, int r) {
        int s1 = matrix.size();
        if (s1 - 2 * r == 0) return;
        int s2 = matrix[0].size();
        if (s2 - 2 * r == 0) return;
        else if (s1 - 2 * r == 1) {
            for (int i = r; i <= s2 - r - 1; ++i) {
                seq.push_back(matrix[r][i]);
            }
        } else if (s2 - 2 * r == 1) {
            for (int i = r; i <= s1 - r - 1; ++i) {
                seq.push_back(matrix[i][r]);
            }
        } else {
            for (int i = r; i < s2 - r - 1; ++i) {
                seq.push_back(matrix[r][i]);
            }
            for (int i = r; i < s1 - r - 1; ++i) {
                seq.push_back(matrix[i][s2 - r - 1]);
            }
            for (int i = s2 - r - 1; i > r; --i) {
                seq.push_back(matrix[s1 - r - 1][i]);
            }
            for (int i = s1 - r - 1; i > r; --i) {
                seq.push_back(matrix[i][r]);
            }
            sub(matrix, seq, r + 1);
        }
    }

public:
    vector<int> spiralOrder(vector<vector<int>>& matrix) {
        vector<int> seq;
        sub(matrix, seq, 0);
        return seq;
    }
};