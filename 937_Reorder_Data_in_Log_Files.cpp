class Solution {
public:
    vector<string> reorderLogFiles(vector<string>& logs) {
        stable_sort(logs.begin(), logs.end(), [](const string & a, const string & b) {
            int ai = 0, bi = 0;
            while (a[ai] != ' ') ++ai;
            while (b[bi] != ' ') ++bi;
            if (isdigit(a[ai + 1])) return false;
            if (isdigit(b[bi + 1])) return true;
            for (int i = 1; ai + i < a.size() || bi + i < b.size(); ++i) {
                if (bi + i == b.size()) return false;
                if (ai + i == a.size()) return true;
                if (a[ai + i] < b[bi + i]) return true;
                if (a[ai + i] > b[bi + i]) return false;
            }
            for (int i = 0; true; ++i) {
                if (i == bi) return false;
                if (i == ai) return true;
                if (a[i] < b[i]) return true;
                if (a[i] > b[i]) return false;
            }
            return false;
        });
        return logs;
    }
};