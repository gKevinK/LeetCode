class Solution {
public:
    string reorganizeString(string S) {
        priority_queue<pair<int, char>> pq;
        vector<int> vc(26, 0);
        for (char & c : S)
            vc[c - 'a']++;
        for (int i = 0; i < 26; i++) {
            if (vc[i] > (S.size() + 1) / 2) return "";
            if (vc[i]) pq.push({ vc[i], i + 'a' });
        }
        string r;
        pair<int, char> p1, p2;
        while (!pq.empty()) {
            p2 = pq.top();
            pq.pop();
            r.push_back(p2.second);
            if (p1.first > 0)
                pq.push(p1);
            p1 = p2;
            p1.first--;
        }
        return r;
    }
};