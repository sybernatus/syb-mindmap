# syb-mindmap README

This extension provides a way to visualize and edit mind maps as code.


## Features

- Open mindmap view in a new tab using `ctrl + shift + P` then `syb mindmap`.
- Open `json` or `yaml` file and start editing the mindmap.

### Json format

```json
{
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

### YAML format
```yaml
data:
  text: My Mindmap
  children:
    - text: My first element
      children:
        - text: View my topic
```

\!\[feature X\]\(images/feature-x.png\)

> Tip: Many popular extensions utilize animations. This is an excellent way to show off your extension! We recommend short, focused animations that are easy to follow.

[//]: # (## Extension Settings)

[//]: # ()
[//]: # (Include if your extension adds any VS Code settings through the `contributes.configuration` extension point.)

[//]: # ()
[//]: # (For example:)

[//]: # ()
[//]: # (This extension contributes the following settings:)

[//]: # ()
[//]: # (* `myExtension.enable`: Enable/disable this extension.)

[//]: # (* `myExtension.thing`: Set to `blah` to do something.)


**Enjoy!**
