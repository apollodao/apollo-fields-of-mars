schema:
	cd contracts/martian-field && cargo schema --target-dir .

ts-types:
	cd apollo-scripts && npx json-schema-to-typescript 