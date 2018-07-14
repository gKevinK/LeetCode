class Solution {
public:
    int nthSuperUglyNumber(int n, vector<int>& primes) {
        vector<int> vt(primes.size(), 0), vn = { 1 };
        while (vn.size() < n) {
            int m = INT_MAX;
            vector<int> vt2;
            for (int i = 0; i < primes.size(); i++) {
                int cur = vn[vt[i]] * primes[i];
                if (cur < m) {
                    m = cur;
                    vt2 = { i };
                } else if (cur == m)
                    vt2.push_back(i);
            }
            vn.push_back(m);
            for (int & i : vt2)
                vt[i]++;
            // for (int & i : vn)
            //     cout << i << " ";
            // cout << endl;
        }
        return vn[n - 1];
    }
};