class Solution {
    void f(string& s, map<pair<int, int>, vector<int>>& m, int a, int b) {
        if (m.find({ a, b }) != m.end()) return;
        vector<int> v;
        for (int i = a; i < b; i++) {
            if (s[i] == '+' || s[i] == '-' || s[i] == '*') {
                f(s, m, a, i);
                f(s, m, i + 1, b);
                for (int& j : m[{ a, i }]) {
                    for (int& k : m[{ i + 1, b }]) {
                        if (s[i] == '+') v.push_back(j + k);
                        if (s[i] == '-') v.push_back(j - k);
                        if (s[i] == '*') v.push_back(j * k);
                    }
                }
            }
        }
        // cout << a << " " << b << " " << s.substr(a, b - a) << endl;
        if (v.empty())
            v.push_back(stoi(s.substr(a, b - a)));
        m[{ a, b }] = v;
    }
public:
    vector<int> diffWaysToCompute(string input) {
        map<pair<int, int>, vector<int>> m;
        f(input, m, 0, input.size());
        return m[{ 0, input.size() }];
    }
};