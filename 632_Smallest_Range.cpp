class Solution {
public:
    vector<int> smallestRange(vector<vector<int>>& nums) {
        using t3 = tuple<int, int, int>;
        int right = -1000000;
        priority_queue<t3, vector<t3>, greater<t3>> pq;
        for (int i = 0; i < nums.size(); ++i) {
            pq.push({ nums[i][0], i, 0 });
            right = max(right, nums[i][0]);
        }
        vector<int> r = { -1000000, right };
        while (true) {
            auto top = pq.top();
            int v = get<0>(top), i = get<1>(top), j = get<2>(top);
            if (right - v < r[1] - r[0])
                r = { v, right };
            if (j + 1 == nums[i].size())
                break;
            pq.pop();
            right = max(right, nums[i][j + 1]);
            pq.push({ nums[i][j + 1], i, j + 1 });
        }
        return r;
    }
};