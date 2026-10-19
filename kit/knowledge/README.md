# Knowledge templates

Installed by `pf knowledge init`. It creates only missing files and never
overwrites anything. `base/` applies to every repo. `stacks/<name>/` adds the
stack's handbook chapters and areas. Paths under `docs/` are mapped to the
module's `docs_root`. Placeholders: `{{PRODUCT}}`, `{{MODULE}}`, `{{AREA}}`,
`{{STACK}}`, `{{DATE}}`.

The structure is distilled from the Daria Connect repositories:
- Back-End and Federation: `spring-kotlin-service`
- Client: `kmp-android-client`
- Front-End: `typescript-web`
- Product: `product-docs`
