use diesel::{PgConnection, RunQueryDsl};

use crate::models::{Program, Project, Round, Vote};
use crate::schema::program::dsl::*;
use crate::schema::project::dsl::*;
use crate::schema::round::dsl::*;
use crate::schema::vote::dsl::*;
use crate::schema::{program, project, round, vote};

pub fn new_program(conn: &mut PgConnection, data: Program) {
    let programs = vec![data];
    insert_programs(conn, programs);
}

pub fn new_round(conn: &mut PgConnection, data: Round) {
    let rounds = vec![data];
    insert_rounds(conn, rounds);
}

pub fn new_project(conn: &mut PgConnection, data: Project) {
    let projects = vec![data];
    insert_projects(conn, projects);
}

pub fn new_vote(conn: &mut PgConnection, data: Vote) {
    let votes = vec![data];
    insert_votes(conn, votes);
}

pub fn new_projects(conn: &mut PgConnection, data: Vec<Project>) {
    let chunk_size = 1000;
    let mut projects = data;

    while projects.len() > chunk_size {
        let (chunk, rest) = projects.split_at(chunk_size);
        insert_projects(conn, chunk.to_vec());
        projects = rest.to_vec();
    }

    insert_projects(conn, projects);
}

pub fn new_rounds(conn: &mut PgConnection, data: Vec<Round>) {
    let chunk_size = 1000;
    let mut rounds = data;

    while rounds.len() > chunk_size {
        let (chunk, rest) = rounds.split_at(chunk_size);
        insert_rounds(conn, chunk.to_vec());
        rounds = rest.to_vec();
    }

    insert_rounds(conn, rounds);
}

pub fn new_programs(conn: &mut PgConnection, data: Vec<Program>) {
    let chunk_size = 1000;
    let mut programs = data;

    while programs.len() > chunk_size {
        let (chunk, rest) = programs.split_at(chunk_size);
        insert_programs(conn, chunk.to_vec());
        programs = rest.to_vec()
    }

    insert_programs(conn, programs);
}

pub fn new_votes(conn: &mut PgConnection, data: Vec<Vote>) {
    let chunk_size = 1000;
    let mut votes = data;

    while votes.len() > chunk_size {
        let (chunk, rest) = votes.split_at(chunk_size);
        insert_votes(conn, chunk.to_vec());
        votes = rest.to_vec()
    }

    insert_votes(conn, votes);
}

fn insert_rounds(conn: &mut PgConnection, data: Vec<Round>) {
    // insert into round table, ignore duplicates
    diesel::insert_into(round::table)
        .values(&data)
        .on_conflict(round::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new round");
}

fn insert_programs(conn: &mut PgConnection, data: Vec<Program>) {
    // insert into program table, ignore duplicates
    diesel::insert_into(program::table)
        .values(&data)
        .on_conflict(program::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new program");
}

fn insert_votes(conn: &mut PgConnection, data: Vec<Vote>) {
    // insert into vote table, ignore duplicates
    diesel::insert_into(vote::table)
        .values(&data)
        .on_conflict(vote::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new vote");
}

fn insert_projects(conn: &mut PgConnection, data: Vec<Project>) {
    // insert into project table, ignore duplicates
    diesel::insert_into(project::table)
        .values(&data)
        .on_conflict(project::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new project");
}

pub async fn get_programs(conn: &mut PgConnection) -> Vec<Program> {
    program
        .load::<Program>(conn)
        .expect("Error loading programs")
}

pub async fn get_rounds(conn: &mut PgConnection) -> Vec<Round> {
    round.load::<Round>(conn).expect("Error loading rounds")
}

pub async fn get_projects(conn: &mut PgConnection) -> Vec<Project> {
    project
        .load::<Project>(conn)
        .expect("Error loading projects")
}

pub async fn get_votes(conn: &mut PgConnection) -> Vec<Vote> {
    vote.load::<Vote>(conn).expect("Error loading votes")
}
