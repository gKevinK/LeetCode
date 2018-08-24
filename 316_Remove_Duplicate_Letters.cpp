// class Solution {
// public:
//     string removeDuplicateLetters(string s) {
//         vector<int> v1(26, 0), v2;
//         for (char & c : s)
//             v1[c - 'a']++;
//         string r;
//         int p = 0;
//         while (p < s.size()) {
//             v2 = v1;
//             for (int i = p; i < s.size(); i++) {
//                 if (v1[s[i] - 'a'] > 0 && (v1[s[p] - 'a'] < 0 || s[i] < s[p])) {
//                     p = i; v2 = v1;
//                 }
//                 if (--v1[s[i] - 'a'] == 0) break;
//             }
//             v1 = v2;
//             if (v1[s[p] - 'a'] <= 0) break;
//             r += s[p];
//             v1[s[p++] - 'a'] = -1;
//         }
//         return r;
//     }
// };

class Solution {
public:
    string removeDuplicateLetters(string s) {
        vector<int> count(26, 0), visited(26, 0);
        for (char & c : s)
            count[c - 'a']++;
        string r;
        for (char & c : s) {
            count[c - 'a']--;
            if (visited[c - 'a']) continue;
            while (!r.empty() && c < r.back() && count[r.back() - 'a'] > 0) {
                visited[r.back() - 'a'] = 0;
                r.pop_back();
            }
            r.push_back(c);
            visited[c - 'a'] = 1;
        }
        return r;
    }
};