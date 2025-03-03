class Solution {
public:
    int numFriendRequests(vector<int>& ages) {
        int s = 0, l = 0, r = 0, n = ages.size();
        sort(ages.begin(), ages.end());
        for (int i = 0; i < n; i++) {
            while (ages[l] <= ages[i] * 0.5 + 7)
                l++;
            while (r < n && ages[r] <= ages[i])
                r++;
            s += max(0, r - l) - (l > i ? 0 : 1);
        }
        return s;
    }
};