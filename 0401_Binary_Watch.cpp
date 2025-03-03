class Solution {
public:
    vector<string> readBinaryWatch(int num) {
        vector<vector<int>> hour = {
            { 0 },
            { 1, 2, 4, 8 },
            { 3, 5, 6, 9, 10 },
            { 7, 11 },
        };
        vector<vector<int>> minute = {
            { 0 },
            { 1, 2, 4, 8, 16, 32 },
            { 3, 5, 6, 9, 10, 12, 17, 18, 20, 24, 33, 34, 36, 40, 48 },
            { 7, 11, 13, 14, 19, 21, 22, 25, 26, 28, 35, 37, 38, 41, 42, 44, 49, 50, 52, 56},
            { 15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58 },
            { 31, 47, 55, 59 },
        };
        vector<string> res;
        for (int i = 0; i <= min(num, 3) && num - i >= 0 ; i++) {
            if (num - i < 6) {
                for (int & h : hour[i]) {
                    for (int & m : minute[num - i]) {
                        res.push_back(to_string(h) + ":" + (m < 10 ? "0" + to_string(m) : to_string(m)));
                    }
                }
            }
        }
        return res;
    }
};