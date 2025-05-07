<div align="center">
    <a href="https://github.com/sybernatus/syb-mindmap">
        <img style="padding-top: 10px; padding-bottom: 0;" width="150px" src="https://raw.githubusercontent.com/sybernatus/syb-mindmap/153a64b9a6e90611a71cc446c311706a3aa21e10/assets/logo/logo_128.png" alt="logo">
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
$schema: https://raw.githubusercontent.com/sybernatus/syb-mindmap/develop/assets/schemas/mindmap.schema.yaml
data:
  text: My Mindmap
  children:
    - text: My first element
      children:
        - text: View my topic
          image: 
            path: "./assets/my-img.png"
            width: 100
```

### Json format

```json
{
  "$schema": "https://raw.githubusercontent.com/sybernatus/syb-mindmap/develop/assets/schemas/mindmap.schema.yaml",
  "data": {
    "text": "My Mindmap",
    "children": [
      {
        "text": "My first element",
        "children": [
          {
            "text": "View my topic",
            "image": {
                "path": "./assets/my-img.png",
                "width": 100
            }
          }
        ]
      }
    ]
  }
}
```


**Enjoy!**
