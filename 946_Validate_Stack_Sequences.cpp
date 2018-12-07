class Solution {
public:
    bool validateStackSequences(vector<int>& pushed, vector<int>& popped) {
        int i1 = 0, i2 = 0;
        stack<int> s;
        if (pushed.size() != popped.size()) return false;
        while (i1 < pushed.size()) {
            s.push(pushed[i1++]);
            while (!s.empty() && i2 < popped.size() && s.top() == popped[i2]) {
                s.pop();
                i2++;
            }
        }
        return i2 == popped.size();
    }
};