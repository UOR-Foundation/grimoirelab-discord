//! The `perceval_discord` suite: one test per scenario in `features/suites/perceval_discord.feature`.
//!
//! Written by the Prism-DevRel builder from the app's prompt. The theorems are
//! checked by the Lean kernel when `just vv` runs `prismpm verify` on `app/`.
//! What these tests hold fixed is what each theorem is about, so a weakened
//! statement that still verifies fails the gate.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below the root")
        .to_path_buf()
}

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let start = model
        .find(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    let end = model[start..]
        .find("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"));
    &model[start..start + end]
}

fn module(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).expect("the module")
}

/// The condition no request may meet, and the theorem that says none does.
fn pinned(breaks: &str, condition: &str, theorem: &str) {
    let model = module("app/src/PercevalDiscordProofs.lex.tex");
    let found = declaration(&model, breaks);
    assert!(
        found.contains(condition),
        "the condition of {breaks} changed:\n{found}"
    );
    let holds = declaration(&model, theorem);
    let statement = format!("statement: (({breaks} request) = false),");
    assert!(
        holds.contains(&statement),
        "{theorem} must state that no request meets {breaks}:\n{holds}"
    );
}

/// A facet theorem: proved by `decide`, about exactly this predicate.
fn holds(path: &str, theorem: &str, statement: &str) {
    let model = module(path);
    let found = declaration(&model, theorem);
    assert!(
        found.contains(&format!("statement: {statement},")),
        "{theorem} must state {statement}:\n{found}"
    );
    assert!(
        found.contains("proof: decide{}"),
        "{theorem} is proved by decide"
    );
}

/// `PD-01`: The app never answers paging when response is foreign
#[test]
fn perceval_discord_pd_01() {
    pinned(
        "breaksInv001",
        "body: ((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request response) =? (PercevalDiscord::responseForeign))) && ((PercevalDiscord::judge request) =? (PercevalDiscord::answerPaging)))",
        "inv001Holds",
    );
}

/// `PD-02`: The app never answers paging when response is stale
#[test]
fn perceval_discord_pd_02() {
    pinned(
        "breaksInv002",
        "body: ((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request response) =? (PercevalDiscord::responseStale))) && ((PercevalDiscord::judge request) =? (PercevalDiscord::answerPaging)))",
        "inv002Holds",
    );
}

/// `PD-03`: The app never answers paging when returned is above limit
#[test]
fn perceval_discord_pd_03() {
    pinned(
        "breaksInv003",
        "body: ((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request limit) <? (project request returned))) && ((PercevalDiscord::judge request) =? (PercevalDiscord::answerPaging)))",
        "inv003Holds",
    );
}

/// `PD-04`: The app never answers paging when remaining is at most floor and sleep is no
#[test]
fn perceval_discord_pd_04() {
    pinned(
        "breaksInv004",
        "body: (((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request remaining) <=? (project request floor))) && ((project request sleep) =? (PercevalDiscord::sleepNo))) && ((PercevalDiscord::judge request) =? (PercevalDiscord::answerPaging)))",
        "inv004Holds",
    );
}

/// `PD-05`: The app never answers done when state is channel
#[test]
fn perceval_discord_pd_05() {
    pinned(
        "breaksInv005",
        "body: ((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request state) =? (PercevalDiscord::stateChannel))) && ((PercevalDiscord::judge request) =? (PercevalDiscord::answerDone)))",
        "inv005Holds",
    );
}

/// `PD-06`: The app always answers failed when response is error and state is not done and state is not refused
#[test]
fn perceval_discord_pd_06() {
    pinned(
        "breaksInv006",
        "body: ((((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request response) =? (PercevalDiscord::responseError))) && (not ((project request state) =? (PercevalDiscord::stateDone)))) && (not ((project request state) =? (PercevalDiscord::stateRefused)))) && (not ((PercevalDiscord::judge request) =? (PercevalDiscord::answerFailed))))",
        "inv006Holds",
    );
}

/// `PD-07`: The app always answers done when state is done
#[test]
fn perceval_discord_pd_07() {
    pinned(
        "breaksInv007",
        "body: ((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request state) =? (PercevalDiscord::stateDone))) && (not ((PercevalDiscord::judge request) =? (PercevalDiscord::answerDone))))",
        "inv007Holds",
    );
}

/// `PD-08`: The app always answers refused when state is refused
#[test]
fn perceval_discord_pd_08() {
    pinned(
        "breaksInv008",
        "body: ((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request state) =? (PercevalDiscord::stateRefused))) && (not ((PercevalDiscord::judge request) =? (PercevalDiscord::answerRefused))))",
        "inv008Holds",
    );
}

/// `PD-09`: The app always answers failed when state is failed
#[test]
fn perceval_discord_pd_09() {
    pinned(
        "breaksInv009",
        "body: ((((((((project request state) <? 5) && ((project request response) <? 4)) && (1 <=? (project request limit))) && ((project request limit) <=? 100)) && ((project request sleep) <? 2)) && ((project request state) =? (PercevalDiscord::stateFailed))) && (not ((PercevalDiscord::judge request) =? (PercevalDiscord::answerFailed))))",
        "inv009Holds",
    );
}

/// `PD-10`: The architecture description declares every stakeholder, concern, component, edge, viewpoint and view it references
#[test]
fn perceval_discord_pd_10() {
    holds(
        "app/src/Architecture.lex.tex",
        "architectureDescriptionHolds",
        "((architectureValid) = true)",
    );
}

/// `PD-11`: The ISO/IEC 25010 quality model numbers every characteristic, subcharacteristic, requirement and measure, and every parent it names is declared
#[test]
fn perceval_discord_pd_11() {
    holds(
        "app/src/Quality.lex.tex",
        "qualityModelHolds",
        "((qualityValid) = true)",
    );
}

/// `PD-12`: Every compliance obligation is numbered in order and names its evidence
#[test]
fn perceval_discord_pd_12() {
    holds(
        "app/src/Compliance.lex.tex",
        "everyObligationEvidenced",
        "((obligationsEvidenced 0 (obligations)) = true)",
    );
}
