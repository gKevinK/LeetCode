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
    int lt(Point& p1, Point& p2, Point& p3, Point& p4) {
        int x1 = p2.x - p1.x, y1 = p2.y - p1.y;
        int x2 = p4.x - p3.x, y2 = p4.y - p3.y;
        return x1 * y2 - x2 * y1;
    }
public:
    vector<Point> outerTrees(vector<Point>& points) {
        if (points.size() < 4) return points;
        Point ll = points[0];
        for (auto & p : points)
            if (p.y < ll.y || (p.y == ll.y && p.x < ll.x))
                ll = p;
        int n = points.size();
        vector<int> v(n);
        iota(v.begin(), v.end(), 0);
        sort(v.begin(), v.end(), [&](int a, int b) {
            Point p1(points[a].x - ll.x, points[a].y - ll.y);
            Point p2(points[b].x - ll.x, points[b].y - ll.y);
            if (lt(ll, points[a], ll, points[b]) == 0)
                return abs(p1.x) + abs(p1.y) < abs(p2.x) + abs(p2.y);
            else return lt(ll, points[a], ll, points[b]) > 0;
        });
        vector<Point> res;
        for (int i : v) {
            if (res.size() < 2) {
                res.push_back(points[i]);
                continue;
            }
            while (lt(res[res.size() - 2], res[res.size() - 1], res[res.size() - 2], points[i]) < 0)
                res.pop_back();
            res.push_back(points[i]);
        }
        int last = v.size() - 2;
        while (lt(ll, points[v[last]], ll, res.back()) == 0)
            res.push_back(points[v[last--]]);
        return res;
    }
};