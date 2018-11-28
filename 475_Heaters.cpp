class Solution {
public:
    int findRadius(vector<int>& houses, vector<int>& heaters) {
        int r = 0, n = heaters.size(), j = 0;
        sort(houses.begin(), houses.end());
        sort(heaters.begin(), heaters.end());
        for (int i = 0; i < houses.size(); i++) {
            int len = INT_MAX;
            while (j < heaters.size() && heaters[j] < houses[i]) j++;
            if (j < heaters.size()) len = min(len, heaters[j] - houses[i]);
            if (j > 0) len = min(len, houses[i] - heaters[j - 1]);
            r = max(r, len);
        }
        return r;
    }
};