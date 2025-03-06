class Solution {
    int f(map<vector<int>, int>& m) {
        int width = 0;
        int x = -1, y = -1;
        for (auto & p : m) {
            if (p.first[0] >= y) {
                width += y - x;
                x = p.first[0];
                y = p.first[1];
            } else {
                y = max(y, p.first[1]);
            }
        }
        return width + y - x;
    }
public:
    int rectangleArea(vector<vector<int>>& rectangles) {
        sort(rectangles.begin(), rectangles.end());
        long area = 0, width = 0;
        int n = rectangles.size(), x = 0;
        int MOD = 1'000'000'007;
        priority_queue<vector<int>> end;
        map<vector<int>, int> m;
        for (int i = 0; i < n || !end.empty();) {
            if (end.empty() || (i < n && rectangles[i][0] < -end.top()[0])) {
                auto & r = rectangles[i];
                area = (area + (r[0] - x) * width) % MOD;
                ++m[{ r[1], r[3] }];
                end.push({ -r[2], r[1], r[3] });
                x = rectangles[i][0];
                ++i;
            } else {
                auto e = end.top();
                end.pop();
                area = (area + (-e[0] - x) * width) % MOD;
                --m[{ e[1], e[2] }];
                if (m[{ e[1], e[2] }] == 0)
                    m.erase({ e[1], e[2] });
                x = -e[0];
            }
            width = f(m);
        }
        return area % MOD;
    }
};