class Solution {
public:
    int numRabbits(vector<int>& answers) {
        unordered_map<int, int> m;
        for (int & i : answers) {
            m[i]++;
        }
        int s = 0;
        for (auto & i : m) {
            int n = i.first + 1;
            s += (i.second / n + (i.second % n > 0)) * n;
        }
        return s;
    }
};