class Solution {
    int* dp;
    int n;
    
    int f(vector<int>& b, int left, int right, int k) {
        if (left > right) return 0;
        if (dp[(left * n + right) * n + k] > 0) return dp[(left * n + right) * n + k];
        for (; left < right && b[left] == b[left + 1]; left++, k++);
        int r = (k + 1) * (k + 1) + f(b, left + 1, right, 0);
        for (int m = left + 1; m <= right; m++) {
            if (b[m] == b[left]) {
                r = max(r, f(b, left + 1, m - 1, 0) + f(b, m, right, k + 1));
            }
        }
        dp[(left * n + right) * n + k] = r;
        return r;
    }
public:
    int removeBoxes(vector<int>& boxes) {
        n = boxes.size();
        dp = new int[n * n * n];
        return f(boxes, 0, n - 1, 0);
    }
};

// Min memory
/*
class Solution {
    int* dp;
    int n, nx = 100;
    
    int f(vector<int>& b, int left, int right, int k) {
        if (left > right) return 0;
        if (dp[(left * n + right) * n + k] > 0) return dp[(left * n + right) * n + k];
        for (; left < right && b[left] == b[left + 1]; left++, k++);
        int r = (k + 1) * (k + 1) + f(b, left + 1, right, 0);
        for (int m = left + 1; m <= right; m++) {
            if (b[m] == b[left]) {
                r = max(r, f(b, left + 1, m - 1, 0) + f(b, m, right, k + 1));
            }
        }
        dp[(left * n + right) * n + k] = r;
        return r;
    }
public:
    int removeBoxes(vector<int>& boxes) {
        static int* _dp;
        n = boxes.size();
        if (_dp == nullptr)
            _dp = new int[nx * nx * nx]();
        dp = _dp;
        for (int i = 0; i < n * n * n; i++)
            dp[i] = 0;
        return f(boxes, 0, n - 1, 0);
    }
};
*/