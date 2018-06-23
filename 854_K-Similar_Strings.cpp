class Solution {
public:
    int kSimilarity(string A, string B) {
        unordered_set<string> s;
        queue<pair<string, int>> q;
        if (A == B) return 0;
        q.push({ A, 0 });
        s.insert(A);
        while (!q.empty()) {
            string C = move(q.front().first);
            int i = 0;
            while (C[i] == B[i]) i++;
            int l = q.front().second;
            q.pop();
            for (int j = i + 1; j < B.size(); j++) {
                if (C[j] == B[i] && B[i] != B[j]) {
                    swap(C[i], C[j]);
                    if (B == C) return l + 1;
                    if (s.find(C) == s.end()) {
                        s.insert(C);
                        q.push({ C, l + 1 });
                    }
                    swap(C[i], C[j]);
                }
            }
        }
    }
};