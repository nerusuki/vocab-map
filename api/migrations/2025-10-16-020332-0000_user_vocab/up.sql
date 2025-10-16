CREATE TABLE user_vocab (
	"user" int4 NOT NULL,
	vocab int4 NOT NULL,
	PRIMARY KEY ("user", vocab),
	FOREIGN KEY ("user") REFERENCES "user"(id) ON DELETE CASCADE,
	FOREIGN KEY (vocab) REFERENCES vocab(id) ON DELETE CASCADE
);
