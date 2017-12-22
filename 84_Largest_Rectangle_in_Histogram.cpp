static int xxx = []() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(0);
    return 0;
}();

class Solution {
public:
    int largestRectangleArea(vector<int>& heights) {
        int m = 0, r = 0;
        stack<int> s;
        heights.push_back(0);
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
};