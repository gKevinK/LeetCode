class MagicDictionary {
    unordered_map<int, vector<string>> m;
public:
    /** Initialize your data structure here. */
    MagicDictionary() {
        
    }
    
    /** Build a dictionary through a list of words */
    void buildDict(vector<string> dict) {
        for (string & s : dict)
            m[s.size()].push_back(s);
    }
    
    /** Returns if there is any word in the trie that equals to the given word after modifying exactly one character */
    bool search(string word) {
        int len = word.size();
        for (string & s : m[len]) {
            int diff = 0;
            for (int i = 0; i < len; ++i) {
                if (s[i] != word[i]) ++diff;
                if (diff >= 2) break;
            }
            if (diff == 1) return true;
        }
        return false;
    }
};

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * MagicDictionary* obj = new MagicDictionary();
 * obj->buildDict(dict);
 * bool param_2 = obj->search(word);
 */