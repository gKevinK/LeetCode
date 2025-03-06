/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 * class MountainArray {
 *   public:
 *     int get(int index);
 *     int length();
 * };
 */
class Solution {
    int bin(int target, MountainArray &arr, int lo, int lv, int hi, int rv) {
        if (lv < target && rv < target) return -1;
        if (lv > target && rv > target) return -1;
        while (lo <= hi) {
            int mi = (hi - lo) / 2 + lo;
            int mv = arr.get(mi);
            if (mv == target)
                return mi;
            else if ((mv < target) ^ (lv > rv))
                lo = mi + 1;
            else
                hi = mi - 1;
        }
        return -1;
    }
    int find(int target, MountainArray &arr, int lo, int lv, int hi, int rv) {
        if (lv > target && rv > target) return -1;
        if (hi - lo < 2) {
            if (lv == target) return lo;
            if (rv == target) return hi;
            return -1;
        }
        int mi = (hi - lo) / 2 + lo;
        int mv1 = arr.get(mi), mv2 = arr.get(mi + 1);
        if (mv1 < mv2) {
            int l = bin(target, arr, lo, lv, mi, mv1);
            if (l >= 0) return l;
            else return find(target, arr, mi + 1, mv2, hi, rv);
        } else {
            int l = find(target, arr, lo, lv, mi, mv1);
            if (l >= 0) return l;
            else return bin(target, arr, mi + 1, mv2, hi, rv);
        }
    }
public:
    int findInMountainArray(int target, MountainArray &mountainArr) {
        return find(target, mountainArr, 0, mountainArr.get(0), mountainArr.length() - 1, mountainArr.get(mountainArr.length() - 1));
    }
};