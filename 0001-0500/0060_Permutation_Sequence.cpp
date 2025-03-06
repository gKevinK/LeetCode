class Solution {
private:
    int a[10];
    int g[10];

    int pop(int rank) {
        int i = 1, c = 1;
        if (g[i] == 1) {
            c--;
        }
        while (c < rank) {
            i++; c++;
            if (g[i] == 1) {
                c--;
            }
        }
        g[i] = 1;
        return i;
    }

public:
    string getPermutation(int n, int k) {
        string s;
        a[0] = 0;   g[0] = 0;
        a[1] = 1;   g[1] = 0;
        for (int i = 2; i < n+1; i++) {
            a[i] = a[i-1] * i;
            g[i] = 0;
        }
        for (int i = n - 1; i > 0; i--) {
            char c = (char)('0' + pop((k - 1) / a[i] + 1));
            s.push_back(c);
            k = (k - 1) % a[i] + 1;
        }
        s.push_back((char)('0' + pop(1)));
        return s;
    }
};