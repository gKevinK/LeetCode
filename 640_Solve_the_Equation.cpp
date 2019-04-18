class Solution {
public:
    string solveEquation(string equation) {
        int x = 0, d = 0, n = equation.size(), l = 1;
        for (int i = 0, j = 1; i < n; i = j++) {
            if (equation[i] == '=') {
                l = -1;
                i++; j++;
            }
            while (j < n && equation[j] != '+' && equation[j] != '-' && equation[j] != '=')
                j++;
            int sgn = l;
            if (equation[i] == '-')
                sgn = -sgn;
            int start = i + 1 - isdigit(equation[i]);
            if (equation[j - 1] == 'x')
                x += sgn * (j - 1 <= start ? 1 : stoi(equation.substr(start, j - start - 1)));
            else
                d += sgn * stoi(equation.substr(start, j - start));
        }
        
        if (x == 0 && d == 0) return "Infinite solutions";
        else if (x == 0) return "No solution";
        else return "x=" + to_string(- d / x);
    }
};