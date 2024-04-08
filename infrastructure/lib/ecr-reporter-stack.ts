import * as cdk from 'aws-cdk-lib';
import { Duration } from 'aws-cdk-lib';
import * as ecr from 'aws-cdk-lib/aws-ecr';
import { Construct } from 'constructs';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as targets from 'aws-cdk-lib/aws-events-targets';
import * as events from 'aws-cdk-lib/aws-events';

export class EcrReporterStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const ubuntu = new ecr.Repository(this, 'ubuntu', {
      repositoryName: 'ubuntu',
      imageScanOnPush: true
    });

    ubuntu.addLifecycleRule({ maxImageAge: Duration.days(30) });

    const debian = new ecr.Repository(this, 'debian', {
      repositoryName: 'debian',
      imageScanOnPush: true
    });

    debian.addLifecycleRule({ maxImageAge: Duration.days(30) });

    const table = new dynamodb.Table(this, 'ECRImageScans', {
      partitionKey: { name: 'ImageID', type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      tableName: 'ECRImageScans',

      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    table.addGlobalSecondaryIndex({
      indexName: 'StatusIndex',
      partitionKey: { name: 'ScanStatus', type: dynamodb.AttributeType.STRING }
    });

    const initiator = new lambda.Function(this, 'Initiator', {
      runtime: lambda.Runtime.NODEJS_20_X,
      handler: 'index.handler',
      code: lambda.Code.fromAsset('functions/initiator'),
      environment: {
        TABLE_NAME: table.tableName,
        SCANNER_FUNCTION_NAME: 'ScannerWorker'
      }
    });

    table.grantReadWriteData(initiator);

    const rule = new events.Rule(this, 'Rule', {
      schedule: events.Schedule.expression('rate(1 hour)')
    });

    rule.addTarget(new targets.LambdaFunction(initiator));
  }
}

