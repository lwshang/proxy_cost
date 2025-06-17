# `proxy_cost`

A quantitative analysis of proxy canister costs.

## Introduction

There are two canisters:
* `proxy`: A minimal implementation of proxy canister which proxy ingress messages as inter-cansiter calls.
* `dest`: A canister to be called by the `proxy`.

Users can send a `proxy_call` requet to `proxy`, then `proxy` will make an inter-canister call to `dest`.

## Run and Get Data

```sh
cargo test -p experiments --test basic -- --show-output
```

Then the `stdout` will contains the experiment data:

```
1. Specific length inputs, fixed length replies:
7,  11748349
107,  12102510
207,  12461133
307,  12817363
407,  13173603
507,  13529521
607,  13886337
707,  14244261
807,  14599908
907,  14956523
2. Fixed length inputs, specific length replies:
0, 11759224
100, 11883676
200, 12007133
300, 12129298
400, 12252728
500, 12375926
600, 12499781
700, 12624382
800, 12746593
900, 12868260
3. Fixed length inputs, specific length rejects:
0, 11764913
100, 11863050
200, 11973682
300, 12085519
400, 12197268
500, 12308991
600, 12420789
700, 12532391
800, 12644189
900, 12756889
```
