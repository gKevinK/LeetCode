class Solution {
    vector<int> m;
    vector<vector<int>> r;
    int sum = 0;
    
public:
    Solution(vector<vector<int>> rects) {
        r = rects;
        sum = 0;
        for (int i = 0; i < rects.size(); i++) {
            sum += (rects[i][2] - rects[i][0] + 1) * (rects[i][3] - rects[i][1] + 1);
            m.push_back(sum);
        }
    }
    
    vector<int> pick() {
        int t = rand() % sum;
        int i = distance(m.begin(), upper_bound(m.begin(), m.end(), t));
        int x = r[i][2] - r[i][0] + 1, y = r[i][3] - r[i][1] + 1;
        t = rand() % (x * y);
        return { t % x + r[i][0], t / x + r[i][1] };
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(rects);
 * vector<int> param_1 = obj.pick();
 */