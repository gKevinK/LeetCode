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
    int eraseOverlapIntervals(vector<Interval>& intervals) {
        sort(intervals.begin(), intervals.end(), [](const auto & a, const auto & b) { return a.end < b.end; });
        int count = 0, end = INT_MIN;
        for (auto & i : intervals) {
            if (i.start >= end) {
                count++;
                end = i.end;
            }
        }
        return intervals.size() - count;
    }
};