class Solution {
public:
    int countSegments(string s) {
        int count = 0, n = s.size();
        for (int i = 0, j = 0; i < n; i = j) {
            while (j < n && s[j] != ' ') j++;
            if (j > i) count++;
            while (j < n && s[j] == ' ') j++;
        }
        return count;
    }
};