WITH tests AS
(
	SELECT
	test.id,
	test.original,
	test.translation,
	test.phonetic,
	t1.original AS orig_score,
	t1.translation AS trans_score,
	t1.phonetic AS phon_score
	FROM test
	LEFT JOIN (SELECT id, test, MAX(created_at) AS max_created FROM test_result GROUP BY test) t2
	ON t2.test = test.id
	LEFT JOIN (
		SELECT
		id,
		created_at,
		AVG(original) OVER(PARTITION BY test ORDER BY created_at ROWS 3 PRECEDING) AS original,
		AVG(translation) OVER(PARTITION BY test ORDER BY created_at ROWS 3 PRECEDING) AS translation,
		AVG(phonetic) OVER(PARTITION BY test ORDER BY created_at ROWS 3 PRECEDING) AS phonetic
		FROM test_result
	) t1
	ON t1.id = t2.id AND t1.created_at = t2.max_created
	WHERE test.testid = ?
)

SELECT id, original, translation, phonetic FROM tests WHERE id IN
(
	-- the words which are known TODO limit to one randomly sampled
	SELECT id FROM tests WHERE id IN
	(
		SELECT id FROM tests WHERE orig_score = 1 AND (trans_score = 1 OR trans_score IS NULL) AND (phon_score = 1 OR phon_score IS NULL) ORDER BY RANDOM() LIMIT 3
	)

	UNION

	-- the words which are not known yet
	SELECT id FROM tests WHERE orig_score < 1 OR trans_score < 1 OR phon_score < 1 OR (orig_score IS NULL AND (trans_score IS NOT NULL OR phon_score IS NOT NULL))

	UNION

	-- words which have not been seen yet
	SELECT id FROM tests WHERE id IN
	(
		SELECT id FROM tests WHERE orig_score IS NULL AND trans_score IS NULL AND phon_score IS NULL LIMIT NOT EXISTS
		(
			SELECT id FROM tests WHERE orig_score < 1 OR trans_score < 1 OR phon_score < 1
		)
	)
)

ORDER BY RANDOM() LIMIT 1
;
