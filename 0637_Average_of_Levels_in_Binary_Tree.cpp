/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    vector<double> averageOfLevels(TreeNode* root) {
        vector<double> v;
        vector<TreeNode *> n1, n2;
        if (root != NULL)
            n1.push_back(root);
        while (!n1.empty()) {
            int n = 0;
            double s = 0.0;
            for (TreeNode * p : n1) {
                n++;
                s += p->val;
                if (p->left != NULL)
                    n2.push_back(p->left);
                if (p->right != NULL)
                    n2.push_back(p->right);
            }
            v.push_back(s / n);
            n1.clear();
            n1.swap(n2);
        }
        return v;
    }
};