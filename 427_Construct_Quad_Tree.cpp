/*
// Definition for a QuadTree node.
class Node {
public:
    bool val;
    bool isLeaf;
    Node* topLeft;
    Node* topRight;
    Node* bottomLeft;
    Node* bottomRight;

    Node() {}

    Node(bool _val, bool _isLeaf, Node* _topLeft, Node* _topRight, Node* _bottomLeft, Node* _bottomRight) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
};
*/
class Solution {
    Node* c(vector<vector<int>>& grid, int x, int y, int len) {
        if (len == 1) return new Node(grid[x][y], true, nullptr, nullptr, nullptr, nullptr);
        int hlen = len / 2;
        vector<Node *> cs(4);
        cs[0] = c(grid, x, y, hlen);
        cs[1] = c(grid, x, y + hlen, hlen);
        cs[2] = c(grid, x + hlen, y, hlen);
        cs[3] = c(grid, x + hlen, y + hlen, hlen);
        if (cs[0]->isLeaf && cs[1]->isLeaf && cs[2]->isLeaf && cs[3]->isLeaf
            && (cs[0]->val + cs[1]->val + cs[2]->val + cs[3]->val) % 4 == 0) {
            for (auto c : cs) delete c;
            return new Node(grid[x][y], true, nullptr, nullptr, nullptr, nullptr);
        }
        else return new Node(true, false, cs[0], cs[1], cs[2], cs[3]);
    }
public:
    Node* construct(vector<vector<int>>& grid) {
        return c(grid, 0, 0, grid.size());
    }
};