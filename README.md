For `//root/hello:library` I have two features `logging` and `timestamp`

# Method 1
Use target platfrom

```shell
# with only logging feature
buck2 run //hello:main --target-platforms root//platfroms:only_logging  
# with only logging feature
buck2 run //hello:main --target-platforms root//platfroms:only_timestamp
# with logging and timestamp feature
buck2 run //hello:main --target-platforms root//platfroms:logging_timestamp
# have no features
# not given --target-platforms would use the default which is prelude//platforms:default
buck2 run //hello:main
```

## Method2

Pass `--config/-c`


```shell
# with logging and timestamp feature
buck2 run //hello:main2  -c lib_feature.logging=true -c lib_feature.timestamp=true
# with only timestamp feature
buck2 run //hello:main2  -c lib_feature.timestamp=true
# with only logging feature
buck2 run //hello:main2  -c lib_feature.logging=true 
# have no features
buck2 run //hello:main2  
```


----

Can read mroe at https://buck2.build/docs/rule_authors/configurations_by_example/