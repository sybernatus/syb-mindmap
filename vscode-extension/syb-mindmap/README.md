<div align="center">
    <a href="https://github.com/sybernatus/syb-mindmap">
        <img style="padding-top: 10px; padding-bottom: 0;" width="150px" src="https://raw.githubusercontent.com/sybernatus/syb-mindmap/78e196b60fc2f2a11cc53ea77c7d0fd5add1f226/assets/logo/logo.png" alt="logo">
        <h1 style="width: 200px; padding-top: 0">Syb-mindmap</h1>
    </a>
</div>

This extension provides a way to visualize and edit mind maps as code.

## Table of Contents

- [Features](#features)
- [Usage](#usage)
  - [YAML format](#yaml-format)
  - [Json format](#json-format)

## Features

- Open mindmap view in a new tab using `ctrl + shift + P` then `syb mindmap`.
- Open `json` or `yaml` file and start editing the mindmap.

## Usage

### YAML format

```yaml
$schema: https://raw.githubusercontent.com/sybernatus/syb-mindmap/main/assets/schemas/mindmap.schema.yaml
data:
  text: My Mindmap
  children:
    - text: My first element
      children:
        - text: View my topic
```

### Json format

```json
{
  "$schema": "https://raw.githubusercontent.com/sybernatus/syb-mindmap/main/assets/schemas/mindmap.schema.yaml",
  "data": {
    "text": "My Mindmap",
    "children": [
      {
        "text": "My first element",
        "children": [
          {
            "text": "View my topic"
          }
        ]
      }
    ]
  }

}
```


**Enjoy!**
