class Solution {
public:
    int calculate(string s) {
        vector<int> vi;
        vector<char> vo;
        for (char & c : s) {
            if (c == ' ') continue;
            if ('0' <= c && c <= '9') {
                if (vi.size() == vo.size())
                    vi.push_back(0);
                vi.back() = vi.back() * 10 + (c - '0');
            } else {
                vo.push_back(c);
            }
        }
        for (int i = 0; i < vo.size(); i++) {
            if (vo[i] == '*' || vo[i] == '/') {
                for (int j = i; j < vo.size() && (vo[j] == '*' || vo[j] == '/'); j++) {
                    if (vo[j] == '*') vi[i] *= vi[j + 1];
                    else vi[i] /= vi[j + 1];
                    vo[j] = '+';
                    vi[j + 1] = 0;
                }
            }
        }
        for (int i = 0; i < vo.size(); i++) {
            vi[0] += (vo[i] == '+' ? 1 : -1) * vi[i + 1];
        }
        return vi[0];
    }
};