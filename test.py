#!/usr/bin/env python
# encoding: utf-8

import redis
import json

r = redis.Redis(host='localhost', password='root', port=6379, db=0)

data = {
    "type": "email",
    "sender": "lightstrawberry@163.com",
    "receiver": "21828604@qq.com",
    "cc": "",
    "data": {
        "subject": "test",
        "content": "hello world"
    }
}

data = json.dumps(data)

r.publish('notify', data)
