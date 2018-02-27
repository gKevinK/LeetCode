/**
 * Definition for a point.
 * struct Point {
 *     int x;
 *     int y;
 *     Point() : x(0), y(0) {}
 *     Point(int a, int b) : x(a), y(b) {}
 * };
 */
class Solution {
    int GCD(int a, int b) {
        if (b == 0) return a;
        return GCD(b, a % b);
    }
public:
    int maxPoints(vector<Point>& points) {
        int m = 0;
        for (int p1 = 0; p1 < points.size(); p1++) {
            map<pair<int, int>, int> map;
            int v = 0, s = 0;
            for (int p2 = p1 + 1; p2 < points.size(); p2++) {
                int dx = points[p2].x - points[p1].x;
                int dy = points[p2].y - points[p1].y;
                if (dx == 0 && dy == 0) {
                    s++;
                    continue;
                }
                else if (dx == 0) {
                    v++;
                    continue;
                }
                else {
                    int gcd = GCD(abs(dx), abs(dy)) * (dx > 0 ? 1 : -1);
                    dx /= gcd;
                    dy /= gcd;
                }
                map[make_pair(dx, dy)]++;
            }
            int lm = v;
            for (auto & i : map) {
                lm = max(lm, i.second);
            }
            m = max(m, lm + s + 1);
        }
        return m;
    }
};