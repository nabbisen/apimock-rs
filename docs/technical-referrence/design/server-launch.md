# Server launch

## Flow

```
get env args
app `new()`
`init_logger()`
if config file path env arg overwriting
  replace config or default values with env arg
load config
validate
dump config
if port env arg overwriting
  replace config or default values with env arg
start to listen
```

### load config

```
read root config
read related configs and middlewares
for rule sets
  for rules
    compute derived fields
```

### validate

```
root config
validate
for rule sets
  validate
  for rules
    validate
    for whens, responds
      validate
%% note: if any file offered as file path is missing, process will abort
```

### dump config

```
dump root config
for each rule sets
  dump rule set
  for each rules
    dump rule
    for whens, responds
      dump their children and themselves
```
