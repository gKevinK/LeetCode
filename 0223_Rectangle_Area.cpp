class Solution {
public:
    int computeArea(int A, int B, int C, int D, int E, int F, int G, int H) {
        int sum = (C - A) * (D - B);
        if (E >= C || F >= D || G <= A || H <= B);
        else sum -= (min(C, G) - max(A, E)) * (min(D, H) - max(B, F));
        sum += (G - E) * (H - F);
        return sum;
    }
};