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
