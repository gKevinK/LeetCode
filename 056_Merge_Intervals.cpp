class Solution {
private:
     struct cmp {
         bool operator() (const Interval &a, const Interval &b) {
            return a.start < b.start;
         }
     };

public:
    vector<Interval> merge(vector<Interval>& intervals) {
        vector<Interval> v;
        if (intervals.size() == 0) return v;
        sort(intervals.begin(), intervals.end(), cmp());
        for (int i = 0; i < intervals.size(); ++i) {
            if (! v.empty() && intervals[i].start <= v.back().end) {
                v.back().end = max(intervals[i].end, v.back().end);
            } else {
                v.push_back(intervals[i]);
            }
        }
        return v;
    }
};