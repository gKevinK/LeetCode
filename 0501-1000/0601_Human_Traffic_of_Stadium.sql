# Write your MySQL query statement below
SELECT t1.*
FROM stadium t1, stadium t2, stadium t3
WHERE t1.people >= 100 AND t2.people >= 100 AND t3.people >= 100
    AND (  (t1.id + 1 = t2.id AND t1.id + 2 = t3.id)
        OR (t1.id - 1 = t2.id AND t1.id + 1 = t3.id)
        OR (t1.id - 2 = t2.id AND t1.id - 1 = t3.id))
GROUP BY t1.id