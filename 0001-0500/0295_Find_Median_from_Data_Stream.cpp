class MedianFinder {
    vector<int> vs, vl;
    
public:
    /** initialize your data structure here. */
    MedianFinder() {
        make_heap(vs.begin(), vs.end(), less<int>());
        make_heap(vl.begin(), vl.end(), greater<int>());
    }
    
    void addNum(int num) {
        if (vl.empty() || num < vl.front()) {
            vs.push_back(num);
            push_heap(vs.begin(), vs.end());
        } else {
            vl.push_back(num);
            push_heap(vl.begin(), vl.end(), greater<int>());
        }
        if (vs.size() > vl.size() + 1) {
            pop_heap(vs.begin(), vs.end());
            vl.push_back(vs.back());
            push_heap(vl.begin(), vl.end(), greater<int>());
            vs.pop_back();
        } else if (vs.size() < vl.size()) {
            pop_heap(vl.begin(), vl.end(), greater<int>());
            vs.push_back(vl.back());
            push_heap(vs.begin(), vs.end());
            vl.pop_back();
        }
    }
    
    double findMedian() {
        if (vs.size() > vl.size())
            return vs.front();
        else
            return (0.0 + vs.front() + vl.front()) / 2;
    }
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder obj = new MedianFinder();
 * obj.addNum(num);
 * double param_2 = obj.findMedian();
 */