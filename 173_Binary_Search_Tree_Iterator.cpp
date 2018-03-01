/**
 * Definition for binary tree
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class BSTIterator {
    vector<TreeNode *> v;
public:
    BSTIterator(TreeNode *root) {
        while (root) {
            v.push_back(root);
            root = root->left;
        }
    }

    /** @return whether we have a next smallest number */
    bool hasNext() {
        return !v.empty();
    }

    /** @return the next smallest number */
    int next() {
        TreeNode* n = v.back();
        v.pop_back();
        if (n->right) {
            TreeNode* t = n->right;
            while (t) {
                v.push_back(t);
                t = t->left;
            }
        }
        return n->val;
    }
};

/**
 * Your BSTIterator will be called like this:
 * BSTIterator i = BSTIterator(root);
 * while (i.hasNext()) cout << i.next();
 */