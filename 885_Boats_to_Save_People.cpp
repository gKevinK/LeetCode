class Solution {
public:
    int numRescueBoats(vector<int>& people, int limit) {
        int lo = 0, hi = people.size(), res = 0;
        sort(people.begin(), people.end());
        while (lo < hi) {
            hi--;
            res++;
            if (lo != hi && people[lo] + people[hi] <= limit)
                lo++;
        }
        return res;
    }
};