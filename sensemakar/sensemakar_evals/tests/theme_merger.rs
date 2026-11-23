mod common;

use sensemakar::theme_merger::ThemeMerger;
use sensemakar_evals::harness::evaluate_case;
use sensemakar_evals::rubric::RubricBlueprint;
use sensemakar_types::Theme;

#[tokio::test]
async fn bus_duplicates() {
    let blueprint =
        RubricBlueprint::load("blueprints/theme_merger.yaml").expect("failed to load blueprint");
    let case = blueprint.case("bus-duplicates").expect("missing case");
    let candidates: Vec<Theme> = case.load_fixture().expect("failed to load fixture");

    let merger = ThemeMerger::builder().build();
    let merge_result = merger
        .merge_with_model(candidates.clone(), common::candidate_model())
        .await
        .expect("theme merge failed");

    let judge = common::judge();
    let verdict = evaluate_case(&judge, &blueprint.description, &candidates, &merge_result, case)
        .await
        .expect("judge call failed");

    common::record_and_assert(&blueprint.task, &blueprint.description, &case.id, &verdict);
}
