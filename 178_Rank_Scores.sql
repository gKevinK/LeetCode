# Write your MySQL query statement below
SELECT Score, (SELECT count(*) FROM (SELECT DISTINCT Score s FROM Scores) A WHERE s >= Score) Rank
FROM Scores ORDER BY Score DESC