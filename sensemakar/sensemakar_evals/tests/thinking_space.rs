mod common;

use sensemakar::thinking_space::{FollowUpQuestionGenerator, SummaryGenerator};
use sensemakar_evals::harness::evaluate_case;
use sensemakar_evals::rubric::RubricBlueprint;
use sensemakar_types::thinking_space::QuestionChain;

const TOPIC: &str = "A conversation about social prescribing in Scotland";

#[tokio::test]
async fn social_prescribing_followup() {
    let blueprint = RubricBlueprint::load("blueprints/thinking_space_followup.yaml")
        .expect("failed to load blueprint");
    let case = blueprint
        .case("social-prescribing-followup")
        .expect("missing case");
    let question_chain: QuestionChain = case.load_fixture().expect("failed to load fixture");

    let generator = FollowUpQuestionGenerator::builder()
        .topic(TOPIC.into())
        .reading_age_target("9-11 year old".into())
        .build();
    let follow_ups = generator
        .run_with_model(question_chain.clone(), common::candidate_model())
        .await
        .expect("follow-up generation failed");

    let judge = common::judge();
    let verdict = evaluate_case(&judge, &blueprint.description, &question_chain, &follow_ups, case)
        .await
        .expect("judge call failed");

    common::record_and_assert(&blueprint.task, &blueprint.description, &case.id, &verdict);
}

#[tokio::test]
async fn social_prescribing_summary() {
    let blueprint = RubricBlueprint::load("blueprints/thinking_space_summary.yaml")
        .expect("failed to load blueprint");
    let case = blueprint
        .case("social-prescribing-summary")
        .expect("missing case");
    let question_chain: QuestionChain = case.load_fixture().expect("failed to load fixture");

    let generator = SummaryGenerator::builder()
        .topic(TOPIC.into())
        .reading_age_target("9-11 year old".into())
        .build();
    let summary = generator
        .run_with_model(question_chain.clone(), common::candidate_model())
        .await
        .expect("summary generation failed");

    let judge = common::judge();
    let verdict = evaluate_case(&judge, &blueprint.description, &question_chain, &summary, case)
        .await
        .expect("judge call failed");

    common::record_and_assert(&blueprint.task, &blueprint.description, &case.id, &verdict);
}
