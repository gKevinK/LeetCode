class Solution {
public:
    int maximalSquare(vector<vector<char>>& matrix) {
        int s = 0;
        for (int i = 0; i < matrix.size(); i++) {
            for (int j = 0; j < matrix[0].size(); j++) {
                if (i == 0 || j == 0)
                    matrix[i][j] = matrix[i][j] - '0';
                else
                    matrix[i][j] = matrix[i][j] == '1' ?
                        min((int)matrix[i-1][j-1], min((int)matrix[i-1][j], (int)matrix[i][j-1])) + 1 : 0;
                s = max(s, (int)matrix[i][j]);
            }
        }
        return s * s;
    }
};