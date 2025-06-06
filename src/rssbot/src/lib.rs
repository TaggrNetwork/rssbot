use std::{cell::RefCell, collections::VecDeque, time::Duration};

use candid::{CandidType, Principal};
use ic_cdk::{
    api::{call::CallResult, stable},
    spawn,
};
use ic_cdk_timers::set_timer_interval;
use serde::{Deserialize, Serialize};

thread_local! {
    static STATE: RefCell<State> = Default::default();
}

// This is the regular posting interval of the bot.
const POSTING_INTERVAL_MINS: u64 = 30;

fn read<F, R>(f: F) -> R
where
    F: FnOnce(&State) -> R,
{
    STATE.with(|cell| f(&cell.borrow()))
}

fn mutate<F, R>(f: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

mod rss;

#[derive(Default, CandidType, Serialize, Deserialize)]
pub struct State {
    pub message_queue: VecDeque<(String, Option<String>)>,
    pub logs: VecDeque<String>,
    pub last_entry_timestamp: u64,
}

fn schedule_message<T: ToString>(state: &mut State, body: T, realm: Option<String>) {
    state.message_queue.push_back((body.to_string(), realm));
}

async fn send_message<T: ToString>(body: T, realm: Option<String>) -> Result<u64, String> {
    let blobs: Vec<(String, Vec<u8>)> = Default::default();
    let parent: Option<u64> = None;
    let poll: Option<Vec<u8>> = None;
    let result: CallResult<(Result<u64, String>,)> = ic_cdk::call(
        Principal::from_text("6qfxa-ryaaa-aaaai-qbhsq-cai").unwrap(),
        "add_post",
        (body.to_string(), blobs, parent, realm, poll),
    )
    .await;
    result
        .map_err(|err| format!("{:?}", err))
        .and_then(|(val,)| val)
}

// CANISTER METHODS

#[ic_cdk_macros::query]
fn info(opcode: String) -> Vec<String> {
    if opcode == "logs" {
        read(|state| state.logs.iter().cloned().collect::<Vec<_>>())
    } else {
        read(|s| {
            vec![
                format!("Logs: {}", s.logs.len(),),
                format!("RSS Timestamp: {}", s.last_entry_timestamp),
                format!("Message Queue: {}", s.message_queue.len()),
            ]
        })
    }
}

fn set_timer() {
    let _id = set_timer_interval(Duration::from_secs(4 * 60 * 60), || spawn(hourly_tasks()));
    let _id = set_timer_interval(Duration::from_secs(60 * POSTING_INTERVAL_MINS), || {
        spawn(process_one_message())
    });
}

async fn hourly_tasks() {
    log_if_error(rss::go().await);
}

async fn process_one_message() {
    if let Some((message, realm)) = mutate(|state| state.message_queue.pop_front()) {
        if let Err(err) = send_message(&message, realm.clone()).await {
            mutate(|state| {
                let logs = &mut state.logs;
                logs.push_back(format!("Taggr response to message {}: {:?}", message, err));
                state.message_queue.push_front((message, realm));
            })
        }
    }
}

#[ic_cdk_macros::init]
fn init() {
    set_timer();
}

#[ic_cdk_macros::update]
async fn fixture() {}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    let buffer: Vec<u8> = read(|s| bincode::serialize(s)).expect("couldn't serialize the state");
    let writer = &mut stable::StableWriter::default();
    let _ = writer.write(&buffer);
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    let bytes = stable::stable_bytes();
    let state: State = bincode::deserialize(&bytes).unwrap();
    STATE.with(|cell| cell.replace(state));
    set_timer();
}

fn log_if_error<T>(result: Result<T, String>) {
    if let Err(err) = result {
        mutate(|state| state.logs.push_back(format!("Error: {}", err)))
    }
}
