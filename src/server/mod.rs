#[cfg(feature = "server")]
pub mod api;
pub mod models;

use dioxus::prelude::*;
use models::*;


#[server(SearchTeams)]
pub async fn search_teams(search_term: String) -> Result<Vec<Team>, ServerFnError> {
    let teams_opt = api::search_teams(search_term).await;
    if let Some(teams) = teams_opt {
        Ok(teams)
    }
    else {
        Err(ServerFnError::ServerError("search_teams() returned None".into()))
    }
}

#[server(GetFields)]
pub async fn get_fields() -> Result<Vec<Field>, ServerFnError> {
    let fields_opt = api::get_fields().await;
    if let Some(fields) = fields_opt {
        Ok(fields)
    }
    else {
        Err(ServerFnError::ServerError("get_fields() returned None".into()))
    }
}

#[server(GetTeamDetails)]
pub async fn get_team_details(team_url: String) -> Result<TeamDetails, ServerFnError> {
    let result_opt = api::get_team_details(team_url).await;
    if let Some(details) = result_opt {
        Ok(details)
    }
    else {
        Err(ServerFnError::ServerError("get_team_details() returned None".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
