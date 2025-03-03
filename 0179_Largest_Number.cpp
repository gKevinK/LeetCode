class Solution {
public:
    string largestNumber(vector<int>& nums) {
        vector<string> ns;
        for (int & n : nums)
            ns.push_back(to_string(n));
        sort(ns.begin(), ns.end(), [](const string & a, const string & b) { return a + b > b + a; });
        if (ns.front() == "0")
            return "0";
        string s;
        for (const string & n : ns)
            s += n;
        return s;
    }
};