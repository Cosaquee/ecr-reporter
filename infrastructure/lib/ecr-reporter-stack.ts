import * as cdk from 'aws-cdk-lib';

import { Construct } from 'constructs';

export class EcrReporterStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const ecrRepo = new cdk.CfnResource(this, 'EcrRepo', {
      type: 'AWS::ECR::Repository',
      properties: {
        RepositoryName: 'ecr-reporter'
      }
    });
  }
}
