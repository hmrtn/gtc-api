use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::models::{Program, Project, Round, Vote};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn new_program(conn: &mut PgConnection, program: Program) {
    let programs = vec![program];
    insert_programs(conn, programs);
}

pub fn new_round(conn: &mut PgConnection, round: Round) {
    let rounds = vec![round];
    insert_rounds(conn, rounds);
}

pub fn new_project(conn: &mut PgConnection, project: Project) {
    let projects = vec![project];
    insert_projects(conn, projects);
}

pub fn new_vote(conn: &mut PgConnection, vote: Vote) {
    let votes = vec![vote];
    insert_votes(conn, votes);
}

pub fn new_projects(conn: &mut PgConnection, projects: Vec<Project>) {
    let chunk_size = 1000;
    let mut projects = projects;

    while projects.len() > chunk_size {
        let (chunk, rest) = projects.split_at(chunk_size);
        insert_projects(conn, chunk.to_vec());
        projects = rest.to_vec();
    }

    insert_projects(conn, projects);
}

pub fn new_rounds(conn: &mut PgConnection, rounds: Vec<Round>) {
    let chunk_size = 1000;
    let mut rounds = rounds;

    while rounds.len() > chunk_size {
        let (chunk, rest) = rounds.split_at(chunk_size);
        insert_rounds(conn, chunk.to_vec());
        rounds = rest.to_vec();
    }

    insert_rounds(conn, rounds);
}

pub fn new_programs(conn: &mut PgConnection, programs: Vec<Program>) {
    let chunk_size = 1000;
    let mut programs = programs;

    while programs.len() > chunk_size {
        let (chunk, rest) = programs.split_at(chunk_size);
        insert_programs(conn, chunk.to_vec());
        programs = rest.to_vec()
    }

    insert_programs(conn, programs);
}

pub fn new_votes(conn: &mut PgConnection, votes: Vec<Vote>) {
    let chunk_size = 1000;
    let mut votes = votes;

    while votes.len() > chunk_size {
        let (chunk, rest) = votes.split_at(chunk_size);
        insert_votes(conn, chunk.to_vec());
        votes = rest.to_vec()
    }

    insert_votes(conn, votes);
}


fn insert_rounds(conn: &mut PgConnection, rounds: Vec<Round>) {
    use crate::schema::round;
    // insert into round table, ignore duplicates
    diesel::insert_into(round::table)
        .values(&rounds)
        .on_conflict(round::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new round");
}

fn insert_programs(conn: &mut PgConnection, programs: Vec<Program>) {
    use crate::schema::program;
    // insert into program table, ignore duplicates
    diesel::insert_into(program::table)
        .values(&programs)
        .on_conflict(program::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new program");
}

fn insert_votes(conn: &mut PgConnection, votes: Vec<Vote>) {
    use crate::schema::vote;
    // insert into vote table, ignore duplicates
    diesel::insert_into(vote::table)
        .values(&votes)
        .on_conflict(vote::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new vote");
}

fn insert_projects(conn: &mut PgConnection, projects: Vec<Project>) {
    use crate::schema::project;
    // insert into project table, ignore duplicates
    diesel::insert_into(project::table)
        .values(&projects)
        .on_conflict(project::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new project");
}

pub async fn get_programs(conn: &mut PgConnection) -> Vec<Program> {
    use crate::schema::program::dsl::*;
    program.load::<Program>(conn).expect("Error loading programs")
}

pub async fn get_rounds(conn: &mut PgConnection) -> Vec<Round> {
    use crate::schema::round::dsl::*;
    round.load::<Round>(conn).expect("Error loading rounds")
}

pub async fn get_projects(conn: &mut PgConnection) -> Vec<Project> {
    use crate::schema::project::dsl::*;
    project.load::<Project>(conn).expect("Error loading projects")
}

pub async fn get_votes(conn: &mut PgConnection) -> Vec<Vote> {
    use crate::schema::vote::dsl::*;
    vote.load::<Vote>(conn).expect("Error loading votes")
}
