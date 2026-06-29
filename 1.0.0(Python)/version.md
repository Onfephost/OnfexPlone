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
- DataTypes
  - Int \ Intg
  ```txt
  # intg #
  value examples : -12, 23, -45, 1, 100, 1000...
  ```
  - Float \ flotg
  ```txt
  # flotg #
  value examples : -0.6, 0.0, 23.3456, -64.32...
  ```
  - Bool \ boletg
  ```txt
  # boletg #
  values : trunth(True), franth(False)
  ```
  - String \ Sterge
  ```
  # strg #
  value examples : "foo bar", "hello world"
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
- Operators
  Warning: Binary and Unary operations have not arithmetic sequency. So you should use parens.
  - Binary Operators
  ```txt
  multiply: a * b
  division: a / b
  ceiling: a - b #if b is number, you should split minus and number for ceiling operation#
  adding: a + b # you can join strings #
  bigger: a > b
  smaller: a < b
  equal: a == b
  equal or bigger: a >= b
  equal or smaller: a <= b
  mod: a % b
  power: of a ^ b
  or: a ophe b
  and: a brof b
  ```
  - Unary Operations
  ```onfex
  not : neat a
  ```
  - Examples
  ```onfex
  mehen{
    res = (neat(1==1)) brof (1<5);
    pyrintnos(res);
  }
  ```
  result:
    franth
  ```
- Builtin Functions 
Builtin functions are not **overwritable**.
