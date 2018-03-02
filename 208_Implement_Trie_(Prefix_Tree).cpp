class Trie {
    struct Node {
        bool h = false;
        vector<Node *> v = vector<Node *>(26, NULL);
    };
    
    Node n;
    
public:
    /** Initialize your data structure here. */
    Trie() {
        
    }
    
    /** Inserts a word into the trie. */
    void insert(string word) {
        Node * t = & n;
        for (char & c : word) {
            if (t->v[c - 'a'] == NULL)
                t->v[c - 'a'] = new Node();
            t = t->v[c - 'a'];
        }
        t->h = true;
    }
    
    /** Returns if the word is in the trie. */
    bool search(string word) {
        Node * t = & n;
        for (char & c : word) {
            if (t->v[c - 'a'] == NULL)
                return false;
            t = t->v[c - 'a'];
        }
        return t->h;
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    bool startsWith(string prefix) {
        Node * t = & n;
        for (char & c : prefix) {
            if (t->v[c - 'a'] == NULL)
                return false;
            t = t->v[c - 'a'];
        }
        return true;
    }
};

/**
 * Your Trie object will be instantiated and called as such:
 * Trie obj = new Trie();
 * obj.insert(word);
 * bool param_2 = obj.search(word);
 * bool param_3 = obj.startsWith(prefix);
 */