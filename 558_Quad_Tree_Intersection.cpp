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
public:
    Node* intersect(Node* quadTree1, Node* quadTree2) {
        if (quadTree1->isLeaf)
            return quadTree1->val ? quadTree1 : quadTree2;
        else if (quadTree2->isLeaf)
            return quadTree2->val ? quadTree2 : quadTree1;
        Node* tl = intersect(quadTree1->topLeft,  quadTree2->topLeft);
        Node* tr = intersect(quadTree1->topRight, quadTree2->topRight);
        Node* bl = intersect(quadTree1->bottomLeft,  quadTree2->bottomLeft);
        Node* br = intersect(quadTree1->bottomRight, quadTree2->bottomRight);
        bool val = tl->val;
        if (tl->isLeaf && tr->isLeaf && bl->isLeaf && br->isLeaf
            && tr->val == val && bl->val == val && br->val == val)
            return new Node(val, true, nullptr, nullptr, nullptr, nullptr);
        return new Node(val, false, tl, tr, bl, br);
    }
};