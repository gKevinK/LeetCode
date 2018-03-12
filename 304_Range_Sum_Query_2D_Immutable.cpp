class NumMatrix {
private:
    vector<vector<int>> sumMatrix;

public:
    NumMatrix(vector<vector<int>> &matrix) {
        vector<int> v;
        v.resize((matrix.empty() ? 0 : matrix[0].size()) + 1);
        sumMatrix.push_back(v);
        for (int i = 0; i < matrix.size(); ++i) {
            sumMatrix.push_back(v);
            for (int j = 0; j < matrix[0].size(); ++j) {
                sumMatrix[i+1][j+1] = matrix[i][j] + sumMatrix[i][j+1] + sumMatrix[i+1][j] - sumMatrix[i][j];
            }
        }
    }

    int sumRegion(int row1, int col1, int row2, int col2) {
        return sumMatrix[row2+1][col2+1] - sumMatrix[row1][col2+1] - sumMatrix[row2+1][col1] + sumMatrix[row1][col1];
    }
};

// Your NumMatrix object will be instantiated and called as such:
// NumMatrix numMatrix(matrix);
// numMatrix.sumRegion(0, 1, 2, 3);
// numMatrix.sumRegion(1, 2, 3, 4);