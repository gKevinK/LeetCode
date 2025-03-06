/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Codec {
public:

    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        vector<TreeNode*> vt;
        TreeNode* t = root;
        string r = "[";
        while (t || !vt.empty()) {
            if (t == NULL) {
                r += "n,";
                if (vt.empty()) break;
                t = vt.back()->right;
                vt.pop_back();
            } else {
                r += to_string(t->val) + ',';
                vt.push_back(t);
                t = t->left;
            }
        }
        if (r.back() == '[') r += ']';
        else r.back() = ']';
        // cout << r << endl;
        return r;
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        TreeNode* r = NULL;
        TreeNode** t = &r;
        vector<TreeNode*> v;
        if (data == "[]") return r;
        for (int i = 1; i < data.size(); i++) {
            if (data[i] == 'n') {
                t = &(v.back()->right);
                v.pop_back();
                i++;
            } else {
                int n = 0, sgn = 1;
                if (data[i] == '-') {
                    sgn = -1;
                    i++;
                }
                while (isdigit(data[i])) {
                    n *= 10;
                    n += sgn * (data[i++] - '0');
                }
                *t = new TreeNode(n);
                v.push_back(*t);
                t = &((*t)->left);
            }
        }
        return r;
    }
};

// Your Codec object will be instantiated and called as such:
// Codec codec;
// codec.deserialize(codec.serialize(root));