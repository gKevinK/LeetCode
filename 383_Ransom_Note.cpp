// class Solution {
// public:
//     bool canConstruct(string ransomNote, string magazine) {
//         unordered_map<char, int> map;
//         for (char c : magazine) {
//             if (map.find(c) == map.end())
//                 map[c] = 1;
//             else
//                 map[c]++;
//         }
//         for (char c : ransomNote) {
//             if (map.find(c) == map.end() || map[c] == 0)
//                 return false;
//             else
//                 map[c]--;
//         }
//         return true;
//     }
// };

class Solution {
public:
    bool canConstruct(string ransomNote, string magazine) {
        vector<int> v(26, 0);
        for (char c : magazine)
            v[c - 'a']++;
        for (char c : ransomNote)
            v[c - 'a']--;
        for (int i : v) {
            if (i < 0)
                return false;
        }
        return true;
    }
};