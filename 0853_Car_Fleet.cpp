class Solution {
public:
    int carFleet(int target, vector<int>& position, vector<int>& speed) {
        map<int, double> m;
        int n = position.size(), f = 0;
        double l = 0.0;
        for (int i = 0; i < position.size(); i++)
            m[target - position[i]] = (double)(target - position[i]) / speed[i];
        for (auto & p : m) {
            if (p.second > l) {
                f++;
                l = p.second;
            }
        }
        return f;
    }
};