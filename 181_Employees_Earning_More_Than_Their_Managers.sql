# Write your MySQL query statement below
-- SELECT a.Name AS Employee
-- FROM Employee a
-- INNER JOIN Employee b
-- ON a.ManagerId = b.Id
-- WHERE a.Salary > b.Salary

SELECT a.Name AS Employee
FROM Employee a
INNER JOIN (SELECT DISTINCT Id, Salary FROM Employee) b
ON a.ManagerId = b.Id
WHERE a.Salary > b.Salary