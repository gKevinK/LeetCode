// class ExamRoom {
//     int _N;
//     set<vector<int>> si, sh;
    
//     int dist(int l, int r) {
//         if (l == 0) return r - 1;
//         if (r == _N) return r - l - 1;
//         return (r - l - 1) / 2;
//     }
    
// public:
//     ExamRoom(int N) {
//         _N = N;
//         si.insert({ 0, N });
//         sh.insert({ -dist(0, N), 0, N });
//     }
    
//     int seat() {
//         auto v = *sh.begin();
//         sh.erase(sh.begin());
//         si.erase({ v[1], v[2] });
//         int lo = v[1], hi = v[2], r = (hi - lo - 1) / 2 + lo;
//         if (lo == 0) r = 0;
//         else if (hi == _N) r = _N - 1;
//         if (r - lo) {
//             sh.insert({ -dist(lo, r), lo, r });
//             si.insert({ lo, r });
//         }
//         if (hi - r - 1) {
//             sh.insert({ -dist(r + 1, hi), r + 1, hi });
//             si.insert({ r + 1, hi });
//         }
//         return r;
//     }
    
//     void leave(int p) {
//         int lo = p, hi = p + 1;
//         auto it = si.upper_bound({ p, 0 });
//         if (it != si.end()) {
//             if ((*it)[0] == p + 1) {
//                 hi = (*it)[1];
//                 sh.erase({ -dist(p + 1, hi), p + 1, hi });
//                 it = si.erase(it);
//             }
//         }
//         if (it != si.begin()) {
//             --it;
//             if ((*it)[1] == p) {
//                 lo = (*it)[0];
//                 sh.erase({ -dist(lo, p), lo, p });
//                 si.erase(it);
//             }
//         }
//         sh.insert({ -dist(lo, hi), lo, hi });
//         si.insert({ lo, hi });
//     }
// };

class ExamRoom {
    int n;
    set<int> s;

public:
    ExamRoom(int N) {
        n = N;
        s.clear();
    }
    
    int seat() {
        int last = -1, dist = 0, seat = 0;
        for (int i : s) {
            int _d, _s;
            if (last == -1) {
                _d = i;
                _s = 0;
            } else {
                _d = (i - last) / 2;
                _s = last + _d;
            }
            if (_d > dist) {
                dist = _d;
                seat = _s;
            }
            last = i;
        }
        if (last >= 0 && n - 1 - last > dist) {
            seat = n - 1;
        }
        s.insert(seat);
        return seat;
    }
    
    void leave(int p) {
        s.erase(p);
    }
};

/**
 * Your ExamRoom object will be instantiated and called as such:
 * ExamRoom* obj = new ExamRoom(N);
 * int param_1 = obj->seat();
 * obj->leave(p);
 */