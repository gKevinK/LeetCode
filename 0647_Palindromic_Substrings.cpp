class Solution {
public:
    int countSubstrings(string s) {
        int count = 0, n = s.size();
        for (int i = 0, j; i < s.size(); i = j) {
            j = i + 1;
            while (j < n && s[j] == s[i])
                j++;
            count += (j - i) * (j - i + 1) / 2;
            for (int k = 1; i - k >= 0 && j - 1 + k < n && s[i - k] == s[j - 1 + k]; k++)
                count++;
        }
        return count;
    }
};