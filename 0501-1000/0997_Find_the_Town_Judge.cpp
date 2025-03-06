class Solution {
public:
    int findJudge(int N, vector<vector<int>>& trust) {
        vector<int> v(N + 1, 0);
        for (auto & t : trust) {
            v[t[0]] = -1;
            if (v[t[1]] != -1)
                v[t[1]]++;
        }
        for (int i = 1; i <= N; i++)
            if (v[i] == N - 1)
                return i;
        return -1;
    }
};