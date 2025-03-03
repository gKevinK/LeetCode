class Solution {
    bool ends_with(const string & a, const string & b) {
        int i = a.size() - 1, j = b.size() - 1;
        for (; i >= 0 && j >= 0; --i, --j) {
            if (a[i] != b[j]) return false;
        }
        return j < 0;
    }
public:
    int minimumLengthEncoding(vector<string>& words) {
        sort(words.begin(), words.end(), [](const string & a, const string & b) {
            int i = a.size() - 1, j = b.size() - 1;
            for (; i >= 0 && j >= 0; --i, --j) {
                if (a[i] < b[j]) return true;
                if (a[i] > b[j]) return false;
            }
            return i < j;
        });
        int r = words.front().size() + 1;
        for (int i = 1; i < words.size(); ++i) {
            if (ends_with(words[i], words[i - 1])) r -= words[i - 1].size() + 1;
            r += words[i].size() + 1;
        }
        return r;
    }
};