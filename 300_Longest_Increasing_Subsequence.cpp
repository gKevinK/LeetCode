class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        vector<int> v;
        for (int & i : nums) {
            int l = 0, h = v.size();
            while (l < h) {
                int m = (h - l) / 2 + l;
                if (v[m] < i)
                    l = m + 1;
                else
                    h = m;
            }
            if (l == v.size())
                v.push_back(i);
            else
                v[l] = i;
        }
        return v.size();
    }
};