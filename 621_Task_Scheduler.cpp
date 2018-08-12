class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        vector<int> v(26, 0);
        int m = 0;
        for (char & t : tasks) {
            v[t - 'A']++;
            m = max(m, v[t - 'A']);
        }
        int res = (m - 1) * (n + 1);
        for (int & i : v)
            res += (i == m);
        return max(res, (int)tasks.size());
    }
};