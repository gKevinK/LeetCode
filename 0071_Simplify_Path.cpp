class Solution {
public:
    string simplifyPath(string path) {
        vector<string> p;
        for (int i = 1; i < path.size(); i++) {
            while (i < path.size() && path[i] == '/')
                i++;
            if (i == path.size())
                break;
            int j = i;
            while (j < path.size() && path[j] != '/')
                j++;
            if (j - i == 1 && path[i] == '.')
                continue;
            if (j - i == 2 && path[i] == '.' && path[i + 1] == '.') {
                if (!p.empty())
                    p.pop_back();
            } else
                p.push_back(path.substr(i, j - i));
            i = j;
        }
        string r;
        for (string & name : p)
            r += "/" + name;
        return r.empty() ? "/" : r;
    }
};