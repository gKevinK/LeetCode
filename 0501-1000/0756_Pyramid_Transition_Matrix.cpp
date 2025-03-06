class Solution {
    bool f(string bottom, unordered_map<string, vector<char>> & m, int a, string next) {
        if (bottom.size() == 1) return true;
        if (a == bottom.size() - 1) return f(next, m, 0, "");
        for (char & c : m[bottom.substr(a, 2)])
            if (f(bottom, m, a + 1, next + c)) return true;
        return false;
    }
public:
    bool pyramidTransition(string bottom, vector<string>& allowed) {
        unordered_map<string, vector<char>> m;
        unordered_set<string> s;
        s.insert(bottom);
        for (const string & a : allowed)
            m[a.substr(0, 2)].push_back(a[2]);
        return f(bottom, m, 0, "");
    }
};