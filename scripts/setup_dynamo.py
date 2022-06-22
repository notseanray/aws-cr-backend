#!/bin/python
import boto3
from environs import Env
import sys

# parses markdown files and generates tables from them

# TODO
# combine into one bigger script/cli tool

# load env credentials
env = Env()
env.read_env('../.env', False)

def parseType(t):
    t = t.lower()
    if t == "string":
        return "S"
    if t == "bool":
        return None
    if t == "u16" or t == "u32" or t == "u64":
        return "N"
    if "vec" in t:
        return None
    print("found invalid attribute: ", t)
    exit()

pkey = ""
tableList = []
attributes = []
types = []
first = False
start = False
file = open("../db-schema.md")
tableName = ""

def retain(data, removal):
    newData = []
    for d in data:
        if d != removal and "\t" not in d:
            newData.append(d)
    return newData

for line in file.readlines():
    if len(line) < 10:
        start = False
        if first:
            tableList.append((attributes.copy(), types.copy(), tableName))
            attributes.clear()
            types.clear()
        continue
    if "#### " in line:
        name = line.split(" ")[1].replace(":", "")
        tableName = name
        print("\ntable name: ", name)
        continue
    if line.startswith("|"):
        if "primary key" in line:
            if start:
                print("cannot have two primary keys")
                sys.exit()
            start = True
            first = True
            pkey = line.split(" ")[1]
            print("\tprimary key: ", pkey)
        if start:
            lineData = retain(line.split(" "), "")
            attributeName = lineData[1]
            attType = parseType(lineData[3])
            if attType == None:
                continue
            attributes.append(attributeName)
            types.append(attType)
            print("\tattribute: ", attributeName, " \ttype: ", attType)

dynamodb = boto3.resource("dynamodb", region_name="us-east-1")
for table in tableList:
    if len(table) < 1:
        continue
    if len(table[0]) != len(table[1]):
        print("mismatch length in: ", table[2])
        sys.exit()
    start = True
    attrDef = []
    for k, v in zip(table[0], table[1]):
        if start:
            start = False
            continue
        attrDef.append(
            {
                'AttributeName': k,
                'AttributeType': v
            }
        )
    print(attrDef)
    if len(attrDef) < 1:
        continue
    print("\ncreating", table[2])
    # continue
    table = {
            'TableName': table[2],
            'KeySchema': [
                {
                    'AttributeName': table[0][0],
                    'KeyType': 'HASH'  #Partition key
                },
            ],
            'AttributeDefinitions': attrDef,
            'BillingMode': 'PROVISIONED',
            'ProvisionedThroughput': {'ReadCapacityUnits': 2, 'WriteCapacityUnits': 2}
    }
    dtable = dynamodb.create_table(**table)
    print("Table status:", dtable.table_status)
