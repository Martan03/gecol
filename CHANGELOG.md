# gecol changelog

## v0.1.0

### Features

- Add CLI with core actions:
    - `run`: executes the extraction and building pipeline
    - `list`: displays available templates
    - `config`: manages the configuration file
    - `clear-cache`: clears the extracted colors cache
- Add support for both image paths and hex colors as inputs
- Add pipeline modifiers:
    - `--skip-build`: skips the templates building
    - `--extract-only`: prints the extractor color from the image
- Add extraction progress reporting with terminal spinner
- Add configuration file for specifying templates and other functionality
