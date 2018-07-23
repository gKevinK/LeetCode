class Solution {
public:
    int minRefuelStops(int target, int startFuel, vector<vector<int>>& stations) {
        vector<int> fuels = { startFuel };
        stations.push_back({ target, 0 });
        for (int i = 0; i < stations.size(); i++) {
            int mi = stations[i][0], fu = stations[i][1];
            fuels.push_back(-1);
            for (int j = fuels.size() - 1; j > 0; j--) {
                if (fuels[j - 1] >= mi)
                    fuels[j] = max(fuels[j], fuels[j - 1] + fu);
            }
        }
        stations.pop_back();
        for (int i = 0; i < fuels.size(); i++)
            if (fuels[i] >= target)
                return i;
        return -1;
    }
};