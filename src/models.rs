#![allow(non_snake_case)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{program, project, round, vote};

// PROGRAMS
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = program)]
pub struct Program {
    pub id: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProgramsQuery {
    pub programs: Vec<Program>,
}

// PROJECTS
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = project)]
pub struct Project {
    pub id: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectsQuery {
    pub roundProjects: Vec<Project>,
}

// ROUNDS
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = round)]
pub struct Round {
    pub id: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RoundsQuery {
    pub rounds: Vec<Round>,
}

// VOTES
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = vote)]
pub struct Vote {
    pub id: String,
    pub createdAt: String,
    pub amount: String,
    pub from: String,
    pub to: String,
    pub token: String,
    pub version: String,
    pub projectId: Option<String>,
    pub chainId: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct VotesQuery {
    pub qfvotes: Vec<Vote>,
}
