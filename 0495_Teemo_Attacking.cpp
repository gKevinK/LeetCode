class Solution {
public:
    int findPoisonedDuration(vector<int>& timeSeries, int duration) {
        int s = 0;
        for (int i = 0; i < timeSeries.size(); i++) {
            if (i < timeSeries.size() - 1 && timeSeries[i] + duration > timeSeries[i+1])
                s += timeSeries[i+1] - timeSeries[i];
            else
                s += duration;
        }
        return s;
    }
};