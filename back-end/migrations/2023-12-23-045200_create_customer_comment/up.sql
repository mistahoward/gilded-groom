CREATE TABLE customer_comment (
	id INTEGER PRIMARY KEY,
	customer_id INTEGER REFERENCES customer(id) NOT NULL,
	comment_id INTEGER REFERENCES comment(id) NOT NULL
);
