class Solution {
public:
    double new21Game(int N, int K, int W) {
        if (K == 0) return 1.0;
        if (K - 1 + W <= N) return 1.0;
        queue<double> q;
        double s = N - K + 1;
        for (int i = K - 1; i >= 0; i--) {
            q.push(s / W);
            s += q.back();
            if (i + W < K) {
                s -= q.front();
                q.pop();
            } else
                s -= (i + W > N ? 0.0 : 1.0);
        }
        return q.back();
    }
};