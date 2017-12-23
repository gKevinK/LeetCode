class Solution {
private:
    int largestRectangleArea(vector<int> heights) {
        int m = 0, r = 0;
        stack<int> s;
        // heights.push_back(0);
        for (int i = 0; i < heights.size(); i++) {
            if (s.empty() || heights[s.top()] < heights[i]) {
                s.push(i);
                continue;
            }
            while (!s.empty() && heights[s.top()] >= heights[i]) {
                r = s.top();
                int area = heights[r] * (i - r);
                m = area > m ? area : m;
                s.pop();
            }
            heights[r] = heights[i];
            s.push(r);
        }
        return m;
    }
    
public:
    int maximalRectangle(vector<vector<char>>& matrix) {
        if (matrix.size() == 0) return 0;
        int m = 0, n = matrix[0].size();
        vector<int> v(matrix.size() + 1, 0);
        for (int k = 0; k < n; k++) {
            for (int i = 0; i < matrix.size(); i++) {
                if (v[i] == 0) {
                    int h = 0;
                    for (; h+k < n && matrix[i][h+k] == '1'; h++) {}
                    v[i] = h;
                } else {
                    v[i]--;
                }
            }
            int a = largestRectangleArea(v);
            m = a > m ? a : m;
        }
        return m;
    }
};