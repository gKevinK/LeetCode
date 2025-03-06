class Solution {
public:
    int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
        int total = 0, station = 0, minTotal = 0;
        for (int i = 0; i < gas.size(); ++i) {
            total = total + gas[i] - cost[i];
            if (total < minTotal) {
                minTotal = total;
                station = i + 1;
            }
        }
        if (total < 0)
            return -1;
        else
            return (station == gas.size() ? 0 : station);
    }
};