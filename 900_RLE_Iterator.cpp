class RLEIterator {
    vector<int> a;
    int p1 = 0, p2 = 0;
public:
    RLEIterator(vector<int> A) {
        a = A;
        while (p1 * 2 < a.size() && a[p1 * 2] == 0)
            p1++;
    }
    
    int next(int n) {
        int c = -1;
        for (int i = 0; i < n; ) {
            if (p1 * 2 >= a.size()) return -1;
            c = a[p1 * 2 + 1];
            if (n - i >= a[p1 * 2] - p2) {
                i += a[p1 * 2] - p2;
                p1++;
                p2 = 0;
                while (p1 * 2 < a.size() && a[p1 * 2] == 0)
                    p1++;
            } else {
                p2 += n - i;
                i = n;
            }
        }
        return c;
    }
};

/**
 * Your RLEIterator object will be instantiated and called as such:
 * RLEIterator obj = new RLEIterator(A);
 * int param_1 = obj.next(n);
 */