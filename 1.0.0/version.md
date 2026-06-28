# Version 1.0.0
Onfex has been created.

- [Syntax Writing](#Syntax_Writing)

## Syntax_Writing
- Function Call
```onfex
func(args);
```
- Variable
```onfex
var
```
- Variable Assign
```onfex
var = new;
```
- Main
Main block stores statements for running in writen file.
So when other file mods this file. Main block won't run.
```onfex
mehen{
  ...
}
```
- Mod
```onfex
mot onfexFile;

```
- Mod Features
  - Mod Variable
  ```onfex
  onfexFile->var
  ```
  - Mod Variable Assign
  ```onfex
  onfexFile->var = new;
  ```
  - Mod Function Call
  ```onfex
  onfexFile->func(args);
  ```
  - Mod Rename
  ```onfex
  mot a;
  wrossnosMot a => new;
- Args / Statements Writing
```Onfex
mehen{
  func1(func2());# We have to put semi to end of every firstly parses #
}
```

