CREATE TABLE projection (
    "user" int4 NOT NULL,
    lang language NOT NULL,
    projection jsonb NOT NULL,
    PRIMARY KEY ("user", lang),
    FOREIGN KEY ("user") REFERENCES "user" (id) ON DELETE CASCADE
);
