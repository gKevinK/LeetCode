class WordDictionary {
    struct Node {
        bool h = false;
        vector<Node *> v = vector<Node *>(26, NULL);
    };
    
    Node n;
    
    bool search2(string & word, int r, Node * t) {
        if (t == NULL)
            return false;
        if (r == word.size())
            return t->h;
        if (word[r] == '.') {
            for (Node * p : t->v) {
                if (p && search2(word, r + 1, p))
                    return true;
            }
        } else
            return search2(word, r + 1, t->v[word[r] - 'a']);
        return false;
    }
    
public:
    /** Initialize your data structure here. */
    WordDictionary() {
        
    }
    
    /** Adds a word into the data structure. */
    void addWord(string word) {
        Node * t = & n;
        for (char & c : word) {
            if (t->v[c - 'a'] == NULL)
                t->v[c - 'a'] = new Node();
            t = t->v[c - 'a'];
        }
        t->h = true;
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    bool search(string word) {
        return search2(word, 0, &n);
    }
};

/**
 * Your WordDictionary object will be instantiated and called as such:
 * WordDictionary obj = new WordDictionary();
 * obj.addWord(word);
 * bool param_2 = obj.search(word);
 */