import * as cdk from 'aws-cdk-lib';
import { Duration } from 'aws-cdk-lib';
import * as ecr from 'aws-cdk-lib/aws-ecr';
import { Construct } from 'constructs';

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
  }
}
