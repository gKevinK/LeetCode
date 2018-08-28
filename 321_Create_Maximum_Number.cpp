static int xxx = []() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(0);
    return 0;
}();

class Solution {
    vector<int> maxNum(vector<int>& nums, int k) {
        vector<int> res;
        for (int i = 0; i < nums.size(); i++) {
            while (!res.empty() && res.size() + nums.size() - i > k && res.back() < nums[i])
                res.pop_back();
            if (res.size() < k)
                res.push_back(nums[i]);
        }
        return res;
    }
    vector<int> merge(vector<int>& m1, vector<int>& m2) {
        vector<int> res(m1.size() + m2.size());
        int t1 = 0, t2 = 0;
        while (t1 < m1.size() || t2 < m2.size()) {
            int i = 0;
            while (t1 + i < m1.size() && t2 + i < m2.size() && m1[t1 + i] == m2[t2 + i]) i++;
            if (t2 + i == m2.size() || (t1 + i < m1.size() && m1[t1 + i] > m2[t2 + i])) res[t1 + t2] = (m1[t1++]);
            else res[t1 + t2] = (m2[t2++]);
        }
        return res;
    }
public:
    vector<int> maxNumber(vector<int>& nums1, vector<int>& nums2, int k) {
        int n1 = nums1.size(), n2 = nums2.size();
        vector<int> res;
        for (int k1 = max(0, k - n2); k1 <= min(n1, k); k1++) {
            vector<int> m1 = maxNum(nums1, k1), m2 = maxNum(nums2, k - k1), m3;
            m3 = merge(m1, m2);
            if (m3 > res) res = m3;
        }
        return res;
    }
};