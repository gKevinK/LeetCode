class Solution {
public:
    int monotoneIncreasingDigits(int N) {
        string n = to_string(N);
        int t = -1;
        for (int i = 0; i < (int)n.size() - 1; i++) {
            if (n[i] > n[i + 1]) {
                t = i;
                while (t > 0 && n[t - 1] == n[t])
                    t--;
                break;
            }
        }
        if (t != -1) {
            n[t] -= 1;
            for (int i = t + 1; i < n.size(); i++)
                n[i] = '9';
        }
        return stoi(n);
    }
};