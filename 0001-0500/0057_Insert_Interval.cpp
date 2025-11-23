class Solution {
public:
    vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
        size_t n = intervals.size();
        size_t lo = 0, hi = n;
        while (lo < hi) {
            size_t mi = lo + (hi - lo) / 2;
            if (intervals[mi][1] < newInterval[0])
                lo = mi + 1;
            else
                hi = mi;
        }
        size_t left = lo;
        lo = 0, hi = n;
        while (lo < hi) {
            size_t mi = lo + (hi - lo) / 2;
            if (intervals[mi][0] <= newInterval[1])
                lo = mi + 1;
            else
                hi = mi;
        }
        size_t right = lo;
        if (left < n && intervals[left][0] < newInterval[0])
            newInterval[0] = intervals[left][0];
        if (0 < right && intervals[right - 1][1] > newInterval[1])
            newInterval[1] = intervals[right - 1][1];

        vector<vector<int>> result;
        for (size_t i = 0; i < left; i++)
            result.push_back(intervals[i]);
        result.push_back(newInterval);
        for (size_t i = right; i < n; i++)
            result.push_back(intervals[i]);
        return result;
    }
};