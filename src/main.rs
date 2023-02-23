use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use async_recursion::async_recursion;
use diesel::PgConnection;
use gql_client::Client;
use gtc_api::{establish_connection, new_programs, new_projects, new_rounds, new_votes};
use gtc_api::models::{Program, ProgramsQuery, Project, ProjectsQuery, Round, RoundsQuery, Vote, VotesQuery};

mod models;

pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting server");
    HttpServer::new(|| {
        App::new()
            .service(seed)
            .service(get_rounds)
            .service(get_projects)
            .service(get_votes)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

// an endpoint to trigger seeding
#[get("/seed/{chain_id}")]
async fn seed(chain_id: web::Path<String>) -> impl Responder {
    let pg = &mut establish_connection();
    match &chain_id[..] {
        "fantom_mainnet" => seed_fantom_mainnet(pg).await,
        "fantom_testnet" => seed_fantom_testnet(pg).await,
        "ethereum_goerli" => seed_ethereum_goerli(pg).await,
        "ethereum_mainnet" => seed_ethereum_mainnet(pg).await,
        "optimism_mainnet" => seed_optimism_mainnet(pg).await,
        _ => HttpResponse::BadRequest().body("Invalid chain id"),
    }
}

// an endpoint for getting all rounds
#[get("/rounds")]
async fn get_rounds() -> impl Responder {
    let pg = &mut establish_connection();
    let rounds = gtc_api::get_rounds(pg).await;
    HttpResponse::Ok().json(rounds)
}

// an endpoint for getting all projects
#[get("/projects")]
async fn get_projects() -> impl Responder {
    let pg = &mut establish_connection();
    let projects = gtc_api::get_projects(pg).await;
    HttpResponse::Ok().json(projects)
}

// an endpoint for getting all votes
#[get("/votes")]
async fn get_votes() -> impl Responder {
    let pg = &mut establish_connection();
    let votes = gtc_api::get_votes(pg).await;
    HttpResponse::Ok().json(votes)
}

async fn seed_fantom_mainnet(
    pg: &mut PgConnection,
) -> HttpResponse {
    println!("seeding fantom mainnet data");
    let chain_id = "250";
    let url = dotenv::var("SUBGRAPH_FANTOM_MAINNET_API")
        .expect("SUBGRAPH_FANTOM_MAINNET_API must be set");
    let gql = Client::new(url);
    seed_programs(&gql, pg, chain_id).await;
    seed_rounds(&gql, pg, chain_id).await;
    seed_projects(&gql, pg, chain_id).await;
    seed_votes(&gql, pg, chain_id).await;
    HttpResponse::Ok().body("done: fantom mainnet data seeding")
}

async fn seed_fantom_testnet(
    pg: &mut PgConnection,
) -> HttpResponse {
    println!("seeding fantom testnet data");
    let chain_id = "4002";
    let url = dotenv::var("SUBGRAPH_FANTOM_TESTNET_API")
        .expect("SUBGRAPH_FANTOM_TESTNET_API must be set");
    let gql = Client::new(url);
    seed_programs(&gql, pg, chain_id).await;
    seed_rounds(&gql, pg, chain_id).await;
    seed_projects(&gql, pg, chain_id).await;
    seed_votes(&gql, pg, chain_id).await;
    HttpResponse::Ok().body("done: fantom testnet data seeding")
}

async fn seed_optimism_mainnet(
    pg: &mut PgConnection,
) -> HttpResponse {
    println!("seeding optimism mainnet data");
    let chain_id = "10";
    let url = dotenv::var("SUBGRAPH_OPTIMISM_MAINNET_API")
        .expect("SUBGRAPH_OPTIMISM_MAINNET_API must be set");
    let gql = Client::new(url);
    seed_programs(&gql, pg, chain_id).await;
    seed_rounds(&gql, pg, chain_id).await;
    seed_projects(&gql, pg, chain_id).await;
    seed_votes(&gql, pg, chain_id).await;
    HttpResponse::Ok().body("done: optimism mainnet data seeding")
}

async fn seed_ethereum_goerli(
    pg: &mut PgConnection,
) -> HttpResponse {
    println!("seeding ethereum goerli data");
    let chain_id = "5";
    let url = dotenv::var("SUBGRAPH_ETHEREUM_GOERLI_API")
        .expect("SUBGRAPH_ETHEREUM_GOERLI_API must be set");
    let gql = Client::new(url);
    seed_programs(&gql, pg, chain_id).await;
    seed_rounds(&gql, pg, chain_id).await;
    seed_projects(&gql, pg, chain_id).await;
    seed_votes(&gql, pg, chain_id).await;
    HttpResponse::Ok().body("done: ethereum goerli data seeding")
}

async fn seed_ethereum_mainnet(
    pg: &mut PgConnection,
) -> HttpResponse {
    println!("seeding ethereum mainnet data");
    let chain_id = "1";
    let url = dotenv::var("SUBGRAPH_ETHEREUM_MAINNET_API")
        .expect("SUBGRAPH_ETHEREUM_MAINNET_API must be set");
    let gql = Client::new(url);
    seed_programs(&gql, pg, chain_id).await;
    seed_rounds(&gql, pg, chain_id).await;
    seed_projects(&gql, pg, chain_id).await;
    seed_votes(&gql, pg, chain_id).await;
    HttpResponse::Ok().body("done: ethereum mainnet data seeding")
}

async fn seed_programs(gql: &Client, conn: &mut PgConnection, chain_id: &str) {
    let res = r_query_programs(gql, "").await;
    let res = add_program_chain_id(res, chain_id).await;

    new_programs(conn, res);
}

async fn seed_rounds(gql: &Client, conn: &mut PgConnection, chain_id: &str) {
    let res = r_query_rounds(gql, "").await;
    let res = add_round_chain_id(res, chain_id).await;

    new_rounds(conn, res);
}

async fn seed_projects(gql: &Client, conn: &mut PgConnection, chain_id: &str) {
    let res = r_query_projects(gql, "").await;
    let res = add_project_chain_id(res, chain_id).await;

    new_projects(conn, res);
}

async fn seed_votes(gql: &Client, conn: &mut PgConnection, chain_id: &str) {
    let res = r_query_votes(gql, "").await;
    let res = add_vote_chain_id(res, chain_id).await;

    new_votes(conn, res);
}

#[async_recursion]
async fn r_query_programs(gql: &Client, last_id: &str) -> Vec<Program> {
    let query = format!("
        query GetProgramsQuery {{
            programs(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                createdAt
                updatedAt
            }}
        }}
    ", last_id);

    let res = gql.query::<ProgramsQuery>(&query)
        .await
        .unwrap()
        .expect("Error getting programs");

    let mut programs = res.programs;

    if programs.len() < 1000 {
        return programs;
    }

    let last_id = programs.last().unwrap().id.clone();
    let mut next_programs = Box::pin(r_query_programs(gql, &last_id)).await;

    programs.append(&mut next_programs);

    programs
}

#[async_recursion]
async fn r_query_rounds(gql: &Client, last_id: &str) -> Vec<Round> {
    let query = format!("
        query GetRoundsQuery {{
            rounds(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                createdAt
                updatedAt
            }}
        }}
        ", last_id);

    let res = gql.query::<RoundsQuery>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut rounds = res.rounds;

    if rounds.len() < 1000 {
        return rounds;
    }

    let last_id = rounds.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_rounds(gql, &last_id)).await;

    rounds.append(&mut next_rounds);

    rounds
}

#[async_recursion]
async fn r_query_projects(gql: &Client, last_id: &str) -> Vec<Project> {
    let query = format!("
        query GetProjectQuery {{
            roundProjects(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                createdAt
                updatedAt
            }}
        }}
        ", last_id);

    let res = gql.query::<ProjectsQuery>(&query)
        .await
        .unwrap()
        .expect("Error getting projects");

    let mut projects = res.roundProjects;

    if projects.len() < 1000 {
        return projects;
    }

    let last_id = projects.last().unwrap().id.clone();
    let mut next_projects = Box::pin(r_query_projects(gql, &last_id)).await;

    projects.append(&mut next_projects);

    projects
}

#[async_recursion]
async fn r_query_votes(gql: &Client, last_id: &str) -> Vec<Vote> {
    let query = format!("
        query GetVotesQuery {{
            qfvotes(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                createdAt
                amount
                from
                to
                version
                token
                projectId
            }}
        }}
    ", last_id);

    let res = gql.query::<VotesQuery>(&query)
        .await
        .unwrap()
        .expect("Error getting votes");

    let mut votes = res.qfvotes;

    if votes.len() < 1000 {
        return votes;
    }

    let last_id = votes.last().unwrap().id.clone();
    let mut next_votes = Box::pin(r_query_votes(gql, &last_id)).await;

    votes.append(&mut next_votes);

    votes
}

async fn add_program_chain_id(data: Vec<Program>, chain_id: &str) -> Vec<Program> {
    data.iter().map(|item| {
        let mut item = item.clone();
        item.chainId = Option::from(chain_id.to_string());
        item
    }).collect()
}

async fn add_round_chain_id(data: Vec<Round>, chain_id: &str) -> Vec<Round> {
    data.iter().map(|item| {
        let mut item = item.clone();
        item.chainId = Option::from(chain_id.to_string());
        item
    }).collect()
}

async fn add_project_chain_id(data: Vec<Project>, chain_id: &str) -> Vec<Project> {
    data.iter().map(|item| {
        let mut item = item.clone();
        item.chainId = Option::from(chain_id.to_string());
        item
    }).collect()
}

async fn add_vote_chain_id(data: Vec<Vote>, chain_id: &str) -> Vec<Vote> {
    data.iter().map(|item| {
        let mut item = item.clone();
        item.chainId = Option::from(chain_id.to_string());
        item
    }).collect()
}
