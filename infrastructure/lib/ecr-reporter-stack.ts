import * as cdk from 'aws-cdk-lib';

import { Construct } from 'constructs';

export class EcrReporterStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    new cdk.CfnResource(this, 'debian', {
      type: 'AWS::ECR::Repository',
      properties: {
        RepositoryName: 'debian'
      }
    });

     new cdk.CfnResource(this, 'ubuntu', {
      type: 'AWS::ECR::Repository',
      properties: {
        RepositoryName: 'ubuntu'
      }
    });
  }
}
