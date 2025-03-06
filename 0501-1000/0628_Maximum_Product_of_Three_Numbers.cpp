// class Solution {
// public:
//     int maximumProduct(vector<int>& nums) {
//         int a1 = 0, a2, a3, b1, b2, b3;
//         a1 = a2 = a3 = INT_MIN;
//         b1 = b2 = INT_MAX;
//         for (int & i : nums) {
//             if (a2 >= 0) a3 = max(a3, a2 * i);
//             if (b2 < 0) a3 = max(a3, b2 * i);
//             if (i >= 0) {
//                 if (a1 >= 0) a2 = max(a2, a1 * i);
//                 a1 = max(a1, i);
//                 if (b1 < 0) b2 = min(b2, b1 * i);
//             } else {
//                 if (b1 < 0) a2 = max(a2, b1 * i);
//                 if (a1 >= 0) b2 = min(b2, a1 * i);
//                 b1 = min(b1, i);
//             }
//         }
//         return a3;
//     }
// };

class Solution {
public:
    int maximumProduct(vector<int>& nums) {
        int a1, a2, a3, b1, b2, b3;
        a1 = a2 = a3 = INT_MIN;
        b1 = b2 = INT_MAX;
        for (int & i : nums) {
            if (i > a1) a1 = i;
            if (a1 > a2) swap(a1, a2);
            if (a2 > a3) swap(a2, a3);
            if (i < b1) b1 = i;
            if (b1 < b2) swap(b1, b2);
        }
        return max(a1 * a2 * a3, a3 * b1 * b2);
    }
};