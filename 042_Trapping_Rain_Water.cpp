class Solution {
public:
    int trap(vector<int>& height) {
        stack<int> s;
        int res = 0, t = 0;
        for (int i = 0; i < height.size(); ++i) {
            while (!s.empty()) {
                if (height[s.top()] > height[i]) {
                    res += (i - s.top() - 1) * (height[i] - t); break;
                }
                res += (i - s.top() - 1) * (height[s.top()] - t);
                t = height[s.top()];
                s.pop();
            }
            s.push(i);
            t = 0;
        }
        return res;
    }
};