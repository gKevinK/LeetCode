# Write your MySQL query statement below

# SELECT class FROM courses GROUP BY class HAVING COUNT(DISTINCT student) >= 5

SELECT class
FROM (
    SELECT class, COUNT(DISTINCT student) AS c
    FROM courses
    GROUP BY class) AS t
WHERE t.c >= 5