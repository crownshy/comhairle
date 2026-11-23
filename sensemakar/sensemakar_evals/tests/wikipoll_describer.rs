mod common;

use sensemakar::wikipoll_describer::WikiPollGroupDescriber;
use sensemakar_evals::harness::evaluate_case;
use sensemakar_evals::rubric::RubricBlueprint;
use sensemakar_types::WikiPollData;

#[tokio::test]
async fn south_staffordshire_development() {
    let blueprint = RubricBlueprint::load("blueprints/wikipoll_describer.yaml")
        .expect("failed to load blueprint");
    let case = blueprint
        .case("south-staffordshire-development")
        .expect("missing case");
    let poll_data: WikiPollData = case.load_fixture().expect("failed to load fixture");

    let describer = WikiPollGroupDescriber::builder().build();
    let report = describer
        .run_with_model(&poll_data, common::candidate_model())
        .await
        .expect("wikipoll description failed");

    let judge = common::judge();
    let verdict = evaluate_case(&judge, &blueprint.description, &poll_data, &report, case)
        .await
        .expect("judge call failed");

    common::record_and_assert(&blueprint.task, &blueprint.description, &case.id, &verdict);
}
