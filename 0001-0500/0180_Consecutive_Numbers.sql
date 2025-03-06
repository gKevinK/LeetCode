# Write your MySQL query statement below
SELECT DISTINCT A.Num AS ConsecutiveNums FROM Logs A, Logs B, Logs C WHERE A.Id + 1 = B.Id AND A.Num = B.Num AND A.Id + 2 = C.Id AND A.Num = C.Num