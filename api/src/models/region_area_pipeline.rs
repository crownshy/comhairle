use std::collections::{BTreeMap, BTreeSet, HashMap};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::error::ComhairleError;

pub mod lifecycle;
pub mod reconcile;
pub mod source;
mod state;
pub mod tables;

use tables::{AreaLinkIden, FeatureIden, PipelineIden};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PipelineDefinition {
    pub name: String,
    pub source: PipelineSource,
    pub external_id_field: String,
    pub area_name_field: String,
    pub primary_locale: String,
    pub region_name_fields: BTreeMap<String, String>,
    pub tags: TagRules,
    pub interval_minutes: u32,
    pub attribution: String,
    #[serde(default)]
    pub repair_invalid_geometry: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum PipelineSource {
    Https { url: String },
    UploadedFile { upload_id: Uuid },
}

impl PipelineDefinition {
    pub fn validate(&self) -> Result<(), ComhairleError> {
        if [
            &self.name,
            &self.external_id_field,
            &self.area_name_field,
            &self.primary_locale,
            &self.attribution,
        ]
        .iter()
        .any(|value| value.trim().is_empty())
        {
            return Err(ComhairleError::BadRequest(
                "Pipeline name, fields, primary locale and attribution are required".into(),
            ));
        }
        if self.interval_minutes == 0 {
            return Err(ComhairleError::BadRequest(
                "Pipeline interval must be greater than zero".into(),
            ));
        }
        if !self.region_name_fields.contains_key(&self.primary_locale)
            || self.region_name_fields.iter().any(|(locale, field)| {
                locale.parse::<language_tags::LanguageTag>().is_err() || field.trim().is_empty()
            })
        {
            return Err(ComhairleError::BadRequest(
                "Region name mappings must use valid locales, non-empty fields and include the primary locale".into(),
            ));
        }
        match &self.source {
            PipelineSource::Https { url } => {
                let parsed = reqwest::Url::parse(url).map_err(|_| {
                    ComhairleError::BadRequest("Invalid pipeline source URL".into())
                })?;
                if parsed.scheme() != "https"
                    || parsed.host_str().is_none()
                    || !parsed.username().is_empty()
                    || parsed.password().is_some()
                    || parsed.fragment().is_some()
                {
                    return Err(ComhairleError::BadRequest(
                        "Pipeline sources must use HTTPS without URL credentials".into(),
                    ));
                }
            }
            PipelineSource::UploadedFile { .. } => {}
        }
        self.tags.validate()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TagRules {
    #[serde(default)]
    pub fixed: Vec<String>,
    #[serde(default)]
    pub rules: Vec<TagRule>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TagRule {
    pub conditions: Vec<TagCondition>,
    pub add: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "operator", rename_all = "snake_case", deny_unknown_fields)]
pub enum TagCondition {
    Equals { field: String, value: Value },
    In { field: String, values: Vec<Value> },
    Present { field: String },
}

#[derive(Debug, PartialEq, Eq)]
pub struct TransformedTags {
    pub tags: Vec<String>,
    pub matched_rules: usize,
}

#[derive(Debug)]
pub struct SourceFeature {
    pub external_id: String,
    pub area_name: String,
    pub region_names: BTreeMap<String, String>,
    pub geometry: Value,
    pub properties: HashMap<String, Value>,
    pub tags: TransformedTags,
}

#[derive(Deserialize)]
struct FeatureCollection {
    #[serde(rename = "type")]
    kind: String,
    features: Vec<RawFeature>,
    crs: Option<Value>,
}

#[derive(Deserialize)]
struct RawFeature {
    #[serde(rename = "type")]
    kind: String,
    geometry: Value,
    properties: HashMap<String, Value>,
}

pub fn normalize_snapshot(
    definition: &PipelineDefinition,
    snapshot: &[u8],
) -> Result<Vec<SourceFeature>, ComhairleError> {
    definition.validate()?;
    let collection: FeatureCollection = serde_json::from_slice(snapshot).map_err(|error| {
        ComhairleError::BadRequest(format!("Invalid GeoJSON snapshot: {error}"))
    })?;
    let supported_crs = collection.crs.as_ref().is_none_or(|crs| {
        crs.get("type").and_then(Value::as_str) == Some("name")
            && matches!(
                crs.pointer("/properties/name").and_then(Value::as_str),
                Some("EPSG:4326" | "urn:ogc:def:crs:OGC:1.3:CRS84")
            )
    });
    if collection.kind != "FeatureCollection" || !supported_crs {
        return Err(ComhairleError::BadRequest(
            "The snapshot must be a WGS84 GeoJSON FeatureCollection".into(),
        ));
    }
    if collection.features.is_empty() {
        return Err(ComhairleError::BadRequest(
            "The snapshot contains no features".into(),
        ));
    }

    let mut ids = BTreeSet::new();
    let mut result = Vec::with_capacity(collection.features.len());
    for feature in collection.features {
        if feature.kind != "Feature" {
            return Err(ComhairleError::BadRequest(
                "The snapshot contains an invalid feature".into(),
            ));
        }
        let external_id = match feature.properties.get(&definition.external_id_field) {
            Some(Value::String(value)) if !value.trim().is_empty() => value.clone(),
            Some(Value::Number(value)) => value.to_string(),
            _ => {
                return Err(ComhairleError::BadRequest(format!(
                    "Feature is missing a valid external identifier in {}",
                    definition.external_id_field
                )));
            }
        };
        if !ids.insert(external_id.clone()) {
            return Err(ComhairleError::BadRequest(format!(
                "Duplicate external identifier: {external_id}"
            )));
        }
        let text = |field: &str| {
            feature
                .properties
                .get(field)
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
        };
        let area_name = text(&definition.area_name_field).ok_or_else(|| {
            ComhairleError::BadRequest(format!("Feature {external_id} is missing its area name"))
        })?;
        let region_names: BTreeMap<_, _> = definition
            .region_name_fields
            .iter()
            .filter_map(|(locale, field)| text(field).map(|name| (locale.clone(), name)))
            .collect();
        if !region_names.contains_key(&definition.primary_locale) {
            return Err(ComhairleError::BadRequest(format!(
                "Feature {external_id} is missing its primary Region name"
            )));
        }
        if !valid_geometry_structure(&feature.geometry) {
            return Err(ComhairleError::BadRequest(format!(
                "Feature {external_id} must have valid WGS84 Polygon or MultiPolygon coordinates"
            )));
        }
        let tags = definition.tags.apply(&feature.properties)?;
        result.push(SourceFeature {
            external_id,
            area_name,
            region_names,
            geometry: feature.geometry,
            properties: feature.properties,
            tags,
        });
    }
    Ok(result)
}

fn valid_geometry_structure(geometry: &Value) -> bool {
    fn polygon(value: &Value) -> bool {
        let Some(rings) = value.as_array().filter(|rings| !rings.is_empty()) else {
            return false;
        };
        rings.iter().all(|ring| {
            let Some(points) = ring.as_array().filter(|points| points.len() >= 4) else {
                return false;
            };
            points.first() == points.last()
                && points.iter().all(|point| {
                    let Some(coordinates) = point.as_array().filter(|point| point.len() == 2)
                    else {
                        return false;
                    };
                    matches!(
                        (coordinates[0].as_f64(), coordinates[1].as_f64()),
                        (Some(lng), Some(lat))
                            if lng.is_finite() && lat.is_finite()
                                && (-180.0..=180.0).contains(&lng)
                                && (-90.0..=90.0).contains(&lat)
                    )
                })
        })
    }

    match (
        geometry.get("type").and_then(Value::as_str),
        geometry.get("coordinates"),
    ) {
        (Some("Polygon"), Some(coordinates)) => polygon(coordinates),
        (Some("MultiPolygon"), Some(coordinates)) => coordinates
            .as_array()
            .is_some_and(|polygons| !polygons.is_empty() && polygons.iter().all(polygon)),
        _ => false,
    }
}

impl TagRules {
    pub fn validate(&self) -> Result<(), ComhairleError> {
        validate_tags(&self.fixed)?;
        for rule in &self.rules {
            if rule.conditions.is_empty() || rule.add.is_empty() {
                return Err(ComhairleError::BadRequest(
                    "Each tag rule must have conditions and output tags".into(),
                ));
            }
            validate_tags(&rule.add)?;
            for condition in &rule.conditions {
                condition.validate()?;
            }
        }
        Ok(())
    }

    pub fn apply(
        &self,
        properties: &HashMap<String, Value>,
    ) -> Result<TransformedTags, ComhairleError> {
        self.validate()?;
        let mut tags: BTreeSet<String> = self.fixed.iter().map(|tag| tag.trim().into()).collect();
        let mut matched_rules = 0;
        for rule in &self.rules {
            if rule
                .conditions
                .iter()
                .all(|condition| condition.matches(properties))
            {
                matched_rules += 1;
                tags.extend(rule.add.iter().map(|tag| tag.trim().to_owned()));
            }
        }
        Ok(TransformedTags {
            tags: tags.into_iter().collect(),
            matched_rules,
        })
    }
}

fn validate_tags(tags: &[String]) -> Result<(), ComhairleError> {
    if tags.iter().any(|tag| tag.trim().is_empty()) {
        return Err(ComhairleError::BadRequest("Tags cannot be empty".into()));
    }
    Ok(())
}

impl TagCondition {
    fn validate(&self) -> Result<(), ComhairleError> {
        let (field, values): (&str, &[Value]) = match self {
            Self::Equals { field, value } => (field, std::slice::from_ref(value)),
            Self::In { field, values } => (field, values),
            Self::Present { field } => (field, &[]),
        };
        if field.trim().is_empty() {
            return Err(ComhairleError::BadRequest(
                "A tag condition must name a source property".into(),
            ));
        }
        if matches!(self, Self::In { values, .. } if values.is_empty())
            || values
                .iter()
                .any(|value| !matches!(value, Value::String(_) | Value::Number(_) | Value::Bool(_)))
        {
            return Err(ComhairleError::BadRequest(
                "Tag condition values must be non-empty scalar values".into(),
            ));
        }
        Ok(())
    }

    fn matches(&self, properties: &HashMap<String, Value>) -> bool {
        match self {
            Self::Equals { field, value } => properties.get(field) == Some(value),
            Self::In { field, values } => properties
                .get(field)
                .is_some_and(|value| values.contains(value)),
            Self::Present { field } => properties.get(field).is_some_and(|value| !value.is_null()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn combines_matching_rules_deterministically() -> Result<(), ComhairleError> {
        let rules: TagRules = serde_json::from_value(json!({
            "fixed": [" source:example ", "kind:country"],
            "rules": [
                {"conditions": [
                    {"operator": "equals", "field": "type", "value": "country"},
                    {"operator": "in", "field": "code", "values": ["E", "S"]}
                ], "add": ["kind:country", "scope:example"]},
                {"conditions": [{"operator": "present", "field": "welsh"}], "add": ["has:welsh"]}
            ]
        }))?;
        let properties = HashMap::from([
            ("type".into(), json!("country")),
            ("code".into(), json!("S")),
            ("welsh".into(), Value::Null),
        ]);
        let result = rules.apply(&properties)?;
        assert_eq!(
            result.tags,
            vec!["kind:country", "scope:example", "source:example"]
        );
        assert_eq!(result.matched_rules, 1);
        Ok(())
    }

    #[test]
    fn rejects_invalid_rules_and_unknown_fields() -> Result<(), ComhairleError> {
        let rules: TagRules = serde_json::from_value(json!({
            "rules": [{"conditions": [], "add": ["kind:country"]}]
        }))?;
        assert!(rules.validate().is_err());
        assert!(serde_json::from_value::<TagRules>(json!({"fixed": [], "ruless": []})).is_err());
        let rules: TagRules = serde_json::from_value(json!({
            "rules": [{"conditions": [
                {"operator": "in", "field": "code", "values": []}
            ], "add": ["kind:country"]}]
        }))?;
        assert!(rules.validate().is_err());
        Ok(())
    }

    #[test]
    fn validates_definitions_and_source_syntax() -> Result<(), ComhairleError> {
        let definition = json!({
            "name": "Example",
            "source": {"type": "uploaded_file", "upload_id": "6226a2a3-f323-43f3-93a7-bca0585e940d"},
            "external_id_field": "code",
            "area_name_field": "name",
            "primary_locale": "en",
            "region_name_fields": {"en": "name", "cy": "welsh_name"},
            "tags": {"fixed": ["kind:country"]},
            "interval_minutes": 60,
            "attribution": "Example attribution"
        });
        let valid: PipelineDefinition = serde_json::from_value(definition.clone())?;
        assert!(valid.validate().is_ok());

        for source in [
            json!({"type": "https", "url": "http://example.org/data"}),
            json!({"type": "https", "url": "https://user:password@example.org/data"}),
        ] {
            let mut input = definition.clone();
            input["source"] = source;
            let invalid: PipelineDefinition = serde_json::from_value(input)?;
            assert!(invalid.validate().is_err());
        }
        assert!(
            serde_json::from_value::<PipelineSource>(
                json!({"type": "local_file", "path": "../data.geojson"})
            )
            .is_err()
        );
        assert!(
            serde_json::from_value::<PipelineSource>(
                json!({"type": "uploaded_file", "upload_id": "../data.geojson"})
            )
            .is_err()
        );
        Ok(())
    }

    #[test]
    fn normalizes_source_fields_and_rejects_duplicate_ids() -> Result<(), ComhairleError> {
        let definition: PipelineDefinition = serde_json::from_value(json!({
            "name": "Example",
            "source": {"type": "uploaded_file", "upload_id": "6226a2a3-f323-43f3-93a7-bca0585e940d"},
            "external_id_field": "code",
            "area_name_field": "name",
            "primary_locale": "en",
            "region_name_fields": {"en": "name", "cy": "welsh_name"},
            "tags": {"fixed": ["kind:country"]},
            "interval_minutes": 60,
            "attribution": "Example attribution"
        }))?;
        let feature = json!({
            "type": "Feature",
            "properties": {"code": "001", "name": "Example", "welsh_name": null},
            "geometry": {"type": "Polygon", "coordinates": [[[0,0],[1,0],[1,1],[0,0]]]}
        });
        let valid = json!({"type": "FeatureCollection", "features": [feature.clone()]});
        let result = normalize_snapshot(&definition, &serde_json::to_vec(&valid)?)?;
        assert_eq!(result[0].external_id, "001");
        assert_eq!(result[0].region_names.get("en"), Some(&"Example".into()));
        assert!(!result[0].region_names.contains_key("cy"));
        let duplicate =
            json!({"type": "FeatureCollection", "features": [feature.clone(), feature]});
        assert!(normalize_snapshot(&definition, &serde_json::to_vec(&duplicate)?).is_err());
        assert!(
            normalize_snapshot(
                &definition,
                br#"{"type":"FeatureCollection","features":[]}"#
            )
            .is_err()
        );
        let mut projected = valid.clone();
        projected["crs"] = json!({"type": "name", "properties": {"name": "EPSG:27700"}});
        assert!(normalize_snapshot(&definition, &serde_json::to_vec(&projected)?).is_err());
        projected["crs"] = json!({"type": "name", "properties": {"name": "EPSG:4326"}});
        assert!(normalize_snapshot(&definition, &serde_json::to_vec(&projected)?).is_ok());
        projected["features"][0]["geometry"]["coordinates"] =
            json!([[[0, 0], [1, 0], [1, 1], [0, 2]]]);
        assert!(normalize_snapshot(&definition, &serde_json::to_vec(&projected)?).is_err());
        Ok(())
    }

    #[test]
    #[ignore = "requires a locally supplied pipeline definition and snapshot"]
    fn normalizes_local_snapshot() -> Result<(), Box<dyn std::error::Error>> {
        let definition_path = std::env::var("PIPELINE_TEST_DEFINITION")?;
        let snapshot_path = std::env::var("PIPELINE_TEST_SNAPSHOT")?;
        let expected_count: usize = std::env::var("PIPELINE_TEST_COUNT")?.parse()?;
        let definition: PipelineDefinition =
            serde_json::from_slice(&std::fs::read(definition_path)?)?;
        let snapshot = std::fs::read(snapshot_path)?;
        assert_eq!(
            normalize_snapshot(&definition, &snapshot)?.len(),
            expected_count
        );
        Ok(())
    }
}
