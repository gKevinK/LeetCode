class Solution {
    bool f(vector<double>& v) {
        if (v.size() == 1)
            return abs(v[0] - 24.0f) < 0.000001;
        for (int i = 0; i < v.size(); i++) {
            for (int j = i + 1; j < v.size(); j++) {
                vector<double> v2, v3 = { v[i] + v[j], v[i] - v[j], v[i] * v[j], v[j] - v[i] };
                if (v[i] != 0) v3.push_back(v[j] / v[i]);
                if (v[j] != 0) v3.push_back(v[i] / v[j]);
                for (int k = 0; k < v.size(); k++)
                    if (k != i && k != j)
                        v2.push_back(v[k]);
                for (double & d : v3) {
                    v2.push_back(d);
                    if (f(v2)) return true;
                    v2.pop_back();
                }
            }
        }
        return false;
    }
public:
    bool judgePoint24(vector<int>& nums) {
        vector<double> v;
        for (int & i : nums)
            v.push_back(i);
        return f(v);
    }
};