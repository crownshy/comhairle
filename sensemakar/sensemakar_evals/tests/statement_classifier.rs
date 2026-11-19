mod common;

use sensemakar::statement_classifier::StatementTypeClassifier;
use sensemakar_evals::harness::evaluate_case;
use sensemakar_evals::rubric::RubricBlueprint;
use sensemakar_types::Statement;

#[tokio::test]
async fn mixed_statement_types() {
    let blueprint = RubricBlueprint::load("blueprints/statement_classifier.yaml")
        .expect("failed to load blueprint");
    let case = blueprint.case("mixed-statement-types").expect("missing case");
    let statements: Vec<Statement> = case.load_fixture().expect("failed to load fixture");

    let classifier = StatementTypeClassifier::builder().build();
    let classified = classifier
        .run_with_model(statements.clone(), common::candidate_model())
        .await
        .expect("statement classification failed");

    let judge = common::judge();
    let verdict = evaluate_case(&judge, &blueprint.description, &statements, &classified, case)
        .await
        .expect("judge call failed");

    common::record_and_assert(&blueprint.task, &blueprint.description, &case.id, &verdict);
}
