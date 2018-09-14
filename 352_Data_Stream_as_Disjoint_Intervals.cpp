/**
 * Definition for an interval.
 * struct Interval {
 *     int start;
 *     int end;
 *     Interval() : start(0), end(0) {}
 *     Interval(int s, int e) : start(s), end(e) {}
 * };
 */
class SummaryRanges {
    multiset<pair<int, int>> s;
    
    decltype(s.begin()) merge(decltype(s.begin()) i2) {
        auto i1 = i2;
        i1--;
        if (i2 == s.end() || i1->second + 1 < i2->first) return i1;
        s.insert(i2, { min(i1->first, i2->first), max(i1->second, i2->second) });
        auto it = i2--;
        s.erase(i1);
        s.erase(it);
        return i2;
    }
public:
    /** Initialize your data structure here. */
    SummaryRanges() {
        
    }
    
    void addNum(int val) {
        auto pr = s.upper_bound({ val, INT_MIN });
        s.insert(pr, { val, val });
        pr = merge(pr);
        if (pr != s.begin())
            merge(pr);
    }
    
    vector<Interval> getIntervals() {
        vector<Interval> r;
        for (auto & p : s)
            r.push_back(Interval(p.first, p.second));
        return r;
    }
};

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * SummaryRanges obj = new SummaryRanges();
 * obj.addNum(val);
 * vector<Interval> param_2 = obj.getIntervals();
 */