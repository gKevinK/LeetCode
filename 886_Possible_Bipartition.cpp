class Solution {
public:
    bool possibleBipartition(int N, vector<vector<int>>& dislikes) {
        vector<vector<int>> g(N);
        vector<int> color(N, -1);
        for (auto & dislike : dislikes) {
            g[dislike[0] - 1].push_back(dislike[1] - 1);
            g[dislike[1] - 1].push_back(dislike[0] - 1);
        }
        for (int i = 1; i < N; i++) {
            if (color[i] >= 0) continue;
            color[i] = 1;
            queue<int> q;
            q.push(i);
            while (!q.empty()) {
                int people = q.front();
                q.pop();
                for (auto dislike : g[people]) {
                    if (color[dislike] == color[people]) return false;
                    if (color[dislike] == -1) {
                        color[dislike] = 1 - color[people];
                        q.push(dislike);
                    }
                }
            }
        }
        return true;
    }
};