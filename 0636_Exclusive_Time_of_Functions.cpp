class Solution {
public:
    vector<int> exclusiveTime(int n, vector<string>& logs) {
        vector<int> res(n, 0);
        stack<int> s;
        int last = -1;
        for (string & log : logs) {
            int i = log.find(':');
            bool start = log[i + 1] == 's';
            int id = stoi(log.substr(0, i));
            int time = stoi(log.substr(start ? i + 7 : i + 5));
            if (start) {
                if (!s.empty())
                    res[s.top()] += time - last;
                s.push(id);
                last = time;
            } else {
                res[s.top()] += time - last + 1;
                s.pop();
                last = time + 1;
            }
        }
        return res;
    }
};