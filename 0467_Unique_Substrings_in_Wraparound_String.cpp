class Solution {
public:
    int findSubstringInWraproundString(string p) {
        vector<int> count(26, 0);
        for (int i = 0, j = 1; i < p.size(); i = j++) {
            while (j < p.size() && p[j] - 'a' == (p[j-1] - 'a' + 1) % 26)
                j++;
            for (int k = 0; i + k < j && k < 26; k++)
                count[p[i + k] - 'a'] = max(count[p[i + k] - 'a'], j - i - k);
        }
        return accumulate(count.begin(), count.end(), 0);
    }
};