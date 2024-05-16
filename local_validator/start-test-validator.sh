#!/bin/bash

solana-test-validator \
    --account 3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR test-validator/accounts/3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR.json \
    --account 8QN9yfKqWDoKjvZmqFsgCzAqwZBQuzVVnC388dN5RCPo test-validator/accounts/8QN9yfKqWDoKjvZmqFsgCzAqwZBQuzVVnC388dN5RCPo.json \
    --bpf-program EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj test-validator/programs/EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj.so \
    --bpf-program HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8 test-validator/programs/HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8.so \
    --reset