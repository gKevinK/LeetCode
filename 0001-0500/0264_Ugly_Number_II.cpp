class Solution {
public:
    int nthUglyNumber(int n) {
        int t1 = 0, t2 = 0, t3 = 0;
        vector<int> v;
        v.push_back(1);
        while (v.size() < n) {
            v.push_back(min(v[t1] * 2, min(v[t2] * 3, v[t3] * 5)));
            if (v.back() == v[t1] * 2) t1++;
            if (v.back() == v[t2] * 3) t2++;
            if (v.back() == v[t3] * 5) t3++;
        }
        return v.back();
    }
};