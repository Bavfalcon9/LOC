# LOC

Count the lines of code in your workspace (unsimply)

Made with rustlang using only the standard library (`std`);



## CLI

| **Description:**                            | **Command:**                   |
| ------------------------------------------- | ------------------------------ |
| Count all lines                             | `loc .`                        |
| Count lines not including common IDE files. | `loc . -i ide`                 |
| Custom filter for directories               | `loc . -i "*.vscode" "*.json"` |