class Solution {
public:
    int findMinMoves(vector<int>& machines) {
        int n = machines.size();
        int sum = accumulate(machines.begin(), machines.end(), 0);
        if (sum % n != 0) return -1;
        int c = sum / n, m = 0, f = 0;
        for (int & i : machines) {
            if (i - c > 0)
                m = max(m, i - c);
            f += i - c;
            m = max(m, abs(f));
        }
        return m;
    }
};