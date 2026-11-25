mod common;

use sensemakar::statement_theme_assigner::StatementThemeAssigner;
use sensemakar_evals::harness::evaluate_case;
use sensemakar_evals::rubric::RubricBlueprint;
use sensemakar_types::{Statement, Theme};

/// The task takes statements *and* a theme list, but a rubric case's
/// fixture only covers one input - so the (small, stable) theme list lives
/// here inline, same as `sensemakar::test_helpers::test_themes`.
fn themes() -> Vec<Theme> {
    vec![
        Theme {
            id: "1".into(),
            name: "Transport".into(),
            description: "Statements about transportation".into(),
        },
        Theme {
            id: "2".into(),
            name: "Services".into(),
            description: "Statements about public services".into(),
        },
    ]
}

#[tokio::test]
async fn transport_vs_services() {
    let blueprint = RubricBlueprint::load("blueprints/statement_theme_assigner.yaml")
        .expect("failed to load blueprint");
    let case = blueprint.case("transport-vs-services").expect("missing case");
    let statements: Vec<Statement> = case.load_fixture().expect("failed to load fixture");

    let assigner = StatementThemeAssigner::builder().themes(themes().into()).build();
    let assignments = assigner
        .run_with_model(statements.clone(), common::candidate_model())
        .await
        .expect("theme assignment failed");

    let judge = common::judge();
    let verdict = evaluate_case(&judge, &blueprint.description, &statements, &assignments, case)
        .await
        .expect("judge call failed");

    common::record_and_assert(&blueprint.task, &blueprint.description, &case.id, &verdict);
}
