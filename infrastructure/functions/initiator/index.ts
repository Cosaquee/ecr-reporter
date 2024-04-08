import * as AWS from 'aws-sdk';

const dynamoDB = new AWS.DynamoDB.DocumentClient();
const lambda = new AWS.Lambda();

const { TABLE_NAME, SCANNER_FUNCTION_NAME } = process.env;

if (!TABLE_NAME || !SCANNER_FUNCTION_NAME) {
    throw new Error('TABLE_NAME or SCANNER_FUNCTION_NAME environment variable is required');
}

exports.handler = async (event: any) => {
    const params = {
        TableName: 'ECRImageScans',
        FilterExpression: 'ScanStatus = :status',
        ExpressionAttributeValues: { ':status': 'pending' },
    };

    try {
        const data = await dynamoDB.scan(params).promise();
        for (let item of data.Items || []) {
            await dynamoDB.update({
                TableName: TABLE_NAME,
                Key: { ImageID: item.ImageID },
                UpdateExpression: 'set ScanStatus = :status',
                ExpressionAttributeValues: { ':status': 'in-progress' },
                ReturnValues: 'UPDATED_NEW',
            }).promise();

            await lambda.invoke({
                FunctionName: SCANNER_FUNCTION_NAME, // Ensure this matches your ScannerWorker function name in AWS
                InvocationType: 'Event',
                Payload: JSON.stringify({ ImageID: item.ImageID }),
            }).promise();
        }
    } catch (err) {
        console.error(err);
        throw new Error('Failed to initiate scans');
    }

    return { status: 'Initiated scans for pending images' };
};