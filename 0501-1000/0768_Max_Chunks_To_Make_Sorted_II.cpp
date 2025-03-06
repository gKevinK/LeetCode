class Solution {
public:
    int maxChunksToSorted(vector<int>& arr) {
        int s = 0, m = 0;
        stack<int> si;
        if (arr.empty()) return 0;
        si.push(arr.size() - 1);
        for (int i = arr.size() - 1; i >= 0; i--)
            if (arr[i] < arr[si.top()])
                si.push(i);
        for (int i = 0; i < arr.size(); i++) {
            m = max(m, arr[i]);
            if (!si.empty() && si.top() == i)
                si.pop();
            if (si.empty() || m <= arr[si.top()])
                s++;
        }
        return s;
    }
};