import boto3
from environs import Env 

env = Env()
env.read_env('../.env', False)

db = boto3.resource('dynamodb', 'us-east-1')
tables = list(db.tables.all())
print(tables)
for table in tables:
    response = table.scan()
    data = response['Items']

    while 'LastEvaluatedKey' in response:
        response = table.scan(ExclusiveStartKey=response['LastEvaluatedKey'])
        data.extend(response['Items'])

    for k in data[0]:
        print("\t", k, "\t\t", data[0][k])
