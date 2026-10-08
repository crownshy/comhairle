// openapi-zod-client drops `properties` that sit beside a oneOf/anyOf, which is the shape
// schemars gives a struct with a #[serde(flatten)] enum. Rewriting it as an allOf keeps both.
import { readFile, writeFile } from 'node:fs/promises';

const [inputPath, outputPath] = process.argv.slice(2);
const spec = JSON.parse(await readFile(inputPath, 'utf8'));

for (const schema of Object.values(spec.components?.schemas ?? {})) {
	const unionKey = ['oneOf', 'anyOf'].find((key) => schema[key]);
	if (!unionKey || !schema.properties) continue;

	const { [unionKey]: union, properties, required, ...rest } = schema;
	for (const key of Object.keys(schema)) delete schema[key];
	Object.assign(schema, rest, {
		allOf: [
			{ type: 'object', properties, ...(required && { required }) },
			{ [unionKey]: union }
		]
	});
}

await writeFile(outputPath, JSON.stringify(spec, null, 2));
