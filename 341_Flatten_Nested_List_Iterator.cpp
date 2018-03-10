/**
 * // This is the interface that allows for creating nested lists.
 * // You should not implement it, or speculate about its implementation
 * class NestedInteger {
 *   public:
 *     // Return true if this NestedInteger holds a single integer, rather than a nested list.
 *     bool isInteger() const;
 *
 *     // Return the single integer that this NestedInteger holds, if it holds a single integer
 *     // The result is undefined if this NestedInteger holds a nested list
 *     int getInteger() const;
 *
 *     // Return the nested list that this NestedInteger holds, if it holds a nested list
 *     // The result is undefined if this NestedInteger holds a single integer
 *     const vector<NestedInteger> &getList() const;
 * };
 */
class NestedIterator {
    stack<pair<vector<NestedInteger> *, int>> s;
public:
    NestedIterator(vector<NestedInteger> &nestedList) {
        vector<NestedInteger> * t = &nestedList;
        s.push(make_pair(t, -1));
        while (!s.empty()) {
            s.top().second++;
            if (s.top().second >= s.top().first->size())
                s.pop();
            else {
                if ((*s.top().first)[s.top().second].isInteger())
                    break;
                s.push(make_pair(&(*s.top().first)[s.top().second].getList(), -1));
            }
        }
    }

    int next() {
        vector<NestedInteger> * t = s.top().first;
        int r = s.top().second;
        int res = (*t)[r].getInteger();
        while (!s.empty()) {
            s.top().second++;
            if (s.top().second >= s.top().first->size())
                s.pop();
            else {
                if ((*s.top().first)[s.top().second].isInteger())
                    break;
                s.push(make_pair(&(*s.top().first)[s.top().second].getList(), -1));
            }
        }
        return res;
    }

    bool hasNext() {
        return !s.empty();
    }
};

/**
 * Your NestedIterator object will be instantiated and called as such:
 * NestedIterator i(nestedList);
 * while (i.hasNext()) cout << i.next();
 */