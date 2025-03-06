/**
 * Definition for an interval.
 * struct Interval {
 *     int start;
 *     int end;
 *     Interval() : start(0), end(0) {}
 *     Interval(int s, int e) : start(s), end(e) {}
 * };
 */
class Solution {
public:
    vector<int> findRightInterval(vector<Interval>& intervals) {
        vector<pair<int, int>> m;
        vector<int> res;
        for (int i = 0; i < intervals.size(); i++)
            m.push_back({ intervals[i].start, i });
        sort(m.begin(), m.end());
        for (auto & i : intervals) {
            auto it = lower_bound(m.begin(), m.end(), pair<int, int>({ i.end, INT_MIN }));
            res.push_back(it == m.end() ? -1 : it->second);
        }
        return res;
    }
};