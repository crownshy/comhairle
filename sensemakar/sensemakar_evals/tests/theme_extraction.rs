mod common;

use sensemakar::theme_extraction::ThemeExtractor;
use sensemakar_evals::harness::evaluate_case;
use sensemakar_evals::rubric::RubricBlueprint;
use sensemakar_types::Statement;

#[tokio::test]
async fn transport_grievances() {
    let blueprint = RubricBlueprint::load("blueprints/theme_extraction.yaml")
        .expect("failed to load blueprint");
    let case = blueprint.case("transport-grievances").expect("missing case");
    let statements: Vec<Statement> = case.load_fixture().expect("failed to load fixture");

    let extractor = ThemeExtractor::builder().allow_additional_themes(true).build();
    let themes = extractor
        .run_with_model(statements.clone(), common::candidate_model())
        .await
        .expect("theme extraction failed");

    let judge = common::judge();
    let verdict = evaluate_case(&judge, &blueprint.description, &statements, &themes, case)
        .await
        .expect("judge call failed");

    common::record_and_assert(&blueprint.task, &blueprint.description, &case.id, &verdict);
}
