CREATE TABLE program
(
    "id"        TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId"   TEXT,

    CONSTRAINT "program_pkey" PRIMARY KEY ("id")
);

CREATE TABLE round
(
    "id"        TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId"   TEXT,

    CONSTRAINT "round_pkey" PRIMARY KEY ("id")
);

CREATE TABLE project
(
    "id"        TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId"   TEXT,

    CONSTRAINT "project_pkey" PRIMARY KEY ("id")
);

CREATE TABLE vote
(
    "id"        TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "amount"    TEXT NOT NULL,
    "from"      TEXT NOT NULL,
    "to"        TEXT NOT NULL,
    "token"     TEXT NOT NULL,
    "version"   TEXT NOT NULL,
    "projectId" TEXT,
    "chainId"   TEXT,


    CONSTRAINT "vote_pkey" PRIMARY KEY ("id")
);
