CREATE TABLE "Likes" (
	"id"	TEXT NOT NULL,
	"tweet"	TEXT NOT NULL,
	"user"	TEXT NOT NULL,
	FOREIGN KEY("user") REFERENCES "User"("id"),
	FOREIGN KEY("tweet") REFERENCES "Tweet"("id"),
	PRIMARY KEY("id")
);
