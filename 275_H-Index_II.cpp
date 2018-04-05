// Slow, O(n)
class Solution {
public:
    int hIndex(vector<int>& citations) {
        for (int i = 0; i < citations.size(); i++) {
            if (i >= citations[citations.size() - i - 1])
                return i;
        }
        return citations.size();
    }
};

// Fast, O(log(n))
class Solution {
public:
    int hIndex(vector<int>& citations) {
        int lo = 0, hi = citations.size() - 1, len = citations.size();
        while (lo <= hi) {
            int mi = (hi - lo) / 2 + lo;
            if (len - mi <= citations[mi])
                hi = mi - 1;
            else
                lo = mi + 1;
        }
        return len - lo;
    }
};
