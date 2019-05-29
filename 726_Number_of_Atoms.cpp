class Solution {
    map<string, int> f(string formula) {
        map<string, int> m;
        int n = formula.size();
        for (int i = 0; i < formula.size();) {
            if (formula[i] == '(') {
                int j = i + 1;
                for (int l = 1; j < n && l; ++j) {
                    if (formula[j] == '(') ++l;
                    if (formula[j] == ')') --l;
                }
                auto m2 = f(formula.substr(i + 1, j - i - 2));
                i = j;
                int num = (i < n && isdigit(formula[i])) ? (formula[i++] - '0') : 1;
                while (i < n && isdigit(formula[i])) num = num * 10 + (formula[i++] - '0');
                for (auto & p : m2)
                    m[p.first] += p.second * num;
            } else {
                string atom;
                atom += formula[i++];
                if (i < n && islower(formula[i])) atom += formula[i++];
                int num = (i < n && isdigit(formula[i])) ? (formula[i++] - '0') : 1;
                while (i < n && isdigit(formula[i])) num = num * 10 + (formula[i++] - '0');
                m[atom] += num;
            }
        }
        return m;
    }
public:
    string countOfAtoms(string formula) {
        auto m = f(formula);
        string r;
        for (auto & p : m) {
            r += p.first;
            if (p.second > 1)
                r += to_string(p.second);
        }
        return r;
    }
};