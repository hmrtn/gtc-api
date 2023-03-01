CREATE TABLE programs
(
    "id"        TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId"   TEXT,

    CONSTRAINT "programs_pkey" PRIMARY KEY ("id")
);

CREATE TABLE rounds
(
    "id"        TEXT NOT NULL,
    "payoutStrategy" TEXT NOT NULL, 
    "token" TEXT NOT NULL, 
    "roundStartTime" TEXT NOT NULL, 
    "roundEndTime" TEXT NOT NULL, 
    "applicationsStartTime" TEXT NOT NULL, 
    "applicationsEndTime" TEXT NOT NULL, 
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId"   TEXT,

    CONSTRAINT "rounds_pkey" PRIMARY KEY ("id")
);

CREATE TABLE projects
(
    "id"        TEXT NOT NULL,
    "status"    TEXT NOT NULL,
    "payoutAddress" TEXT NOT NULL,
    "project" TEXT NOT NULL, 
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId"   TEXT,

    CONSTRAINT "projects_pkey" PRIMARY KEY ("id")
);


CREATE TABLE votes
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


    CONSTRAINT "votes_pkey" PRIMARY KEY ("id")
);
