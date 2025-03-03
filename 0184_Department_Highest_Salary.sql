# Write your MySQL query statement below
SELECT D.Name AS Department, E.Name AS Employee, E.Salary
FROM Department D, Employee E
WHERE E.DepartmentID = D.Id AND E.Salary = (SELECT MAX(E2.Salary) FROM Employee E2 WHERE DepartmentId = D.Id);