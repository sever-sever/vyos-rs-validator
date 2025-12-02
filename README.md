# Compare validators
```
root@r14:/home/vyos# time for i in {1..1000}; do sudo ipaddrcheck --is-any-single 127.0.0.1 >/dev/null; done

real	0m3.866s
user	0m1.126s
sys	0m1.937s
root@r14:/home/vyos# 
root@r14:/home/vyos# time for i in {1..1000}; do ./vyos-rs-validator ip-host 127.0.0.1 >/dev/null; done

real	0m0.959s
user	0m0.362s
sys	0m0.632s
root@r14:/home/vyos# 

```
