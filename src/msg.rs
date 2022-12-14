use crate::state::{Ballot, Config, Poll};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePoll {
        poll_id: String,
        question: String,
        options: Vec<String>,
    },
    Vote {
        poll_id: String,
        vote: String,
    },
    DeletePoll {
        poll_id: String,
    },
    RevokeVote {
        poll_id: String,
    },
    ClosePoll {
        poll_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPolls {},
    Poll { poll_id: String },
    Vote { poll_id: String, address: String },
    Config {},
    AllVotesByAddress { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllPollsResponse {
    pub polls: Vec<Poll>,
}

// Previous code omitted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PollResponse {
    pub poll: Option<Poll>,
}

// Previous code omitted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct VoteResponse {
    pub vote: Option<Ballot>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ConfigResponse {
    pub config: Option<Config>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllVotesByAddressResponse {
    pub ballots: Vec<(String, Ballot)>,
}
