class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        if (nums1.size() > nums2.size())
            return findMedianSortedArrays(nums2, nums1);
        int n1 = nums1.size(), n2 = nums2.size(), half = (n1 + n2 + 1) / 2;
        int lo = 0, hi = n1;
        while (true) {
            int m1 = (hi - lo) / 2 + lo, m2 = half - m1;
            if (m1 > 0 && m2 < n2 && nums1[m1 - 1] > nums2[m2])
                hi = m1 - 1;
            else if (m1 < n1 && m2 > 0 && nums2[m2 - 1] > nums1[m1])
                lo = m1 + 1;
            else {
                int lm = INT_MIN, rm = INT_MAX;
                if (m1 > 0) lm = max(lm, nums1[m1 - 1]);
                if (m2 > 0) lm = max(lm, nums2[m2 - 1]);
                if ((n1 + n2) % 2 == 1) return lm;
                if (m1 < n1) rm = min(rm, nums1[m1]);
                if (m2 < n2) rm = min(rm, nums2[m2]);
                return (lm + rm) / 2.0;
            }
        }
    }
};