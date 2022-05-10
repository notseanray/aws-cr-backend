import boto3
from environs import Env 

# TODO
# combine into one bigger script/cli tool

# load env credentials
env = Env()
env.read_env('../.env', False)

dynamodb = boto3.resource("dynamodb", region_name="us-east-1")
table = dynamodb.Table("users")


# create login table
table = dynamodb.create_table(
    TableName='userauth',
    KeySchema=[
        {
            'AttributeName': 'username',
            'KeyType': 'HASH'  #Partition key
        },
    ],
    AttributeDefinitions=[
        {
            'AttributeName': 'password',
            'AttributeType': 'S'
        },
        {
            'AttributeName': 'admin',
            'AttributeType': 'BOOL'
        },
        {
            'AttributeName': 'graduation_year',
            'AttributeType': 'N'
        },
        {
            'AttributeName': 'team',
            'AttributeType': 'L'
        },
        {
            'AttributeName': 'username',
            'AttributeType': 'S'
        },
        {
            'AttributeName': 'creation_timestamp',
            'AttributeType': 'N'
        },
        {
            'AttributeName': 'email',
            'AttributeType': 'S'
        },
        {
            'AttributeName': 'display_name',
            'AttributeType': 'S'
        },
    ],
    #ProvisionedThroughput={
    #    'ReadCapacityUnits': 2,
    #    'WriteCapacityUnits': 2
    #}
)

print("Table status:", table.table_status)

