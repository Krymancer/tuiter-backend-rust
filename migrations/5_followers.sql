CREATE TABLE "Followers" (
	"id"	TEXT NOT NULL,
	"follower"	TEXT NOT NULL,
	"target"	TEXT NOT NULL,
	FOREIGN KEY("follower") REFERENCES "User"("id"),
	FOREIGN KEY("target") REFERENCES "User"("id"),
	PRIMARY KEY("id")
);
