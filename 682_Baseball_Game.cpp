class Solution {
public:
    int calPoints(vector<string>& ops) {
        vector<int> r;
        int p = 0;
        for (const string & op : ops) {
            int s = 0;
            if (op == "C") {
                p -= r.back();
                r.pop_back();
                continue;
            } else if (op == "D") {
                s = 2 * r.back();
            } else if (op == "+") {
                s = r[r.size() - 2] + r[r.size() - 1];
            } else {
                s = stoi(op);
            }
            r.push_back(s);
            p += s;
        }
        return p;
    }
};