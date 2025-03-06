class Solution {
public:
    bool canVisitAllRooms(vector<vector<int>>& rooms) {
        unordered_map<int, bool> m;
        m[0] = true;
        int n = rooms.size();
        queue<int> q;
        q.push(0);
        while (!q.empty()) {
            int c = q.front();
            q.pop();
            for (int & k : rooms[c]) {
                if (!m[k]) {
                    m[k] = true;
                    q.push(k);
                }
            }
        }
        for (int i = 0; i < n; i++)
            if (!m[i])
                return false;
        return true;
    }
};