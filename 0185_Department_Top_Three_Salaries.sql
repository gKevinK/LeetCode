# Write your MySQL query statement below
SELECT D.Name AS Department, E.Name AS Employee, E.Salary
FROM Department D, Employee E
WHERE E.DepartmentID = D.Id
AND (SELECT count(DISTINCT E2.Salary) FROM Employee E2 WHERE E2.DepartmentId = D.Id AND E2.Salary > E.Salary) < 3