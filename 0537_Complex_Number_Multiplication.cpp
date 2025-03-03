class Solution {
    vector<int> parse(string & s) {
        vector<int> r;
        string str = "";
        for (char & c : s) {
            if (c == '+' || c == 'i') {
                r.push_back(stoi(str));
                str = "";
            } else
                str += c;
        }
        return r;
    }
public:
    string complexNumberMultiply(string a, string b) {
        vector<int> va = parse(a), vb = parse(b);
        int ra = va[0] * vb[0] - va[1] * vb[1], rb = va[0] * vb[1] + va[1] * vb[0];
        return to_string(ra) + "+" + to_string(rb) + "i";
    }
};