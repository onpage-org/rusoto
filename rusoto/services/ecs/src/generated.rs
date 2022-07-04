// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl EcsClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "ecs", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>An object representing a container instance or task attachment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Attachment {
    /// <p>Details of the attachment. For elastic network interfaces, this includes the network interface ID, the MAC address, the subnet ID, and the private IPv4 address.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<KeyValuePair>>,
    /// <p>The unique identifier for the attachment.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p> The status of the attachment. Valid values are <code>PRECREATED</code>, <code>CREATED</code>, <code>ATTACHING</code>, <code>ATTACHED</code>, <code>DETACHING</code>, <code>DETACHED</code>, <code>DELETED</code>, and <code>FAILED</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of the attachment, such as <code>ElasticNetworkInterface</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An object representing a change in state for a task attachment.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachmentStateChange {
    /// <p>The Amazon Resource Name (ARN) of the attachment.</p>
    #[serde(rename = "attachmentArn")]
    pub attachment_arn: String,
    /// <p>The status of the attachment.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An attribute is a name-value pair that's associated with an Amazon ECS object. Use attributes to extend the Amazon ECS data model by adding custom metadata to your resources. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes">Attributes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Attribute {
    /// <p>The name of the attribute. The <code>name</code> must contain between 1 and 128 characters. The name may contain letters (uppercase and lowercase), numbers, hyphens (-), underscores (_), forward slashes (/), back slashes (\), or periods (.).</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ID of the target. You can specify the short form ID for a resource or the full Amazon Resource Name (ARN).</p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p>The type of the target to attach the attribute with. This parameter is required if you use the short form ID for a resource instead of the full ARN.</p>
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// <p>The value of the attribute. The <code>value</code> must contain between 1 and 128 characters. It can contain letters (uppercase and lowercase), numbers, hyphens (-), underscores (_), periods (.), at signs (@), forward slashes (/), back slashes (\), colons (:), or spaces. The value can't can't start or end with a space.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The details of the Auto Scaling group for the capacity provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AutoScalingGroupProvider {
    /// <p>The Amazon Resource Name (ARN) that identifies the Auto Scaling group.</p>
    #[serde(rename = "autoScalingGroupArn")]
    pub auto_scaling_group_arn: String,
    /// <p>The managed scaling settings for the Auto Scaling group capacity provider.</p>
    #[serde(rename = "managedScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_scaling: Option<ManagedScaling>,
    /// <p>The managed termination protection setting to use for the Auto Scaling group capacity provider. This determines whether the Auto Scaling group has managed termination protection. The default is disabled.</p> <important> <p>When using managed termination protection, managed scaling must also be used otherwise managed termination protection doesn't work.</p> </important> <p>When managed termination protection is enabled, Amazon ECS prevents the Amazon EC2 instances in an Auto Scaling group that contain tasks from being terminated during a scale-in action. The Auto Scaling group and each instance in the Auto Scaling group must have instance protection from scale-in actions enabled as well. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Auto Scaling User Guide</i>.</p> <p>When managed termination protection is disabled, your Amazon EC2 instances aren't protected from termination when the Auto Scaling group scales in.</p>
    #[serde(rename = "managedTerminationProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_termination_protection: Option<String>,
}

/// <p>The details of the Auto Scaling group capacity provider to update.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AutoScalingGroupProviderUpdate {
    /// <p>The managed scaling settings for the Auto Scaling group capacity provider.</p>
    #[serde(rename = "managedScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_scaling: Option<ManagedScaling>,
    /// <p>The managed termination protection setting to use for the Auto Scaling group capacity provider. This determines whether the Auto Scaling group has managed termination protection.</p> <important> <p>When using managed termination protection, managed scaling must also be used otherwise managed termination protection doesn't work.</p> </important> <p>When managed termination protection is enabled, Amazon ECS prevents the Amazon EC2 instances in an Auto Scaling group that contain tasks from being terminated during a scale-in action. The Auto Scaling group and each instance in the Auto Scaling group must have instance protection from scale-in actions enabled. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Auto Scaling User Guide</i>.</p> <p>When managed termination protection is disabled, your Amazon EC2 instances aren't protected from termination when the Auto Scaling group scales in.</p>
    #[serde(rename = "managedTerminationProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_termination_protection: Option<String>,
}

/// <p>An object representing the networking details for a task or service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsVpcConfiguration {
    /// <p>Whether the task's elastic network interface receives a public IP address. The default value is <code>DISABLED</code>.</p>
    #[serde(rename = "assignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    /// <p><p>The IDs of the security groups associated with the task or service. If you don&#39;t specify a security group, the default security group for the VPC is used. There&#39;s a limit of 5 security groups that can be specified per <code>AwsVpcConfiguration</code>.</p> <note> <p>All specified security groups must be from the same VPC.</p> </note></p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p><p>The IDs of the subnets associated with the task or service. There&#39;s a limit of 16 subnets that can be specified per <code>AwsVpcConfiguration</code>.</p> <note> <p>All specified subnets must be from the same VPC.</p> </note></p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
}

/// <p>The details for a capacity provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CapacityProvider {
    /// <p>The Auto Scaling group settings for the capacity provider.</p>
    #[serde(rename = "autoScalingGroupProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_provider: Option<AutoScalingGroupProvider>,
    /// <p>The Amazon Resource Name (ARN) that identifies the capacity provider.</p>
    #[serde(rename = "capacityProviderArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_arn: Option<String>,
    /// <p>The name of the capacity provider.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the capacity provider. Only capacity providers in an <code>ACTIVE</code> state can be used in a cluster. When a capacity provider is successfully deleted, it has an <code>INACTIVE</code> status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The metadata that you apply to the capacity provider to help you categorize and organize it. Each tag consists of a key and an optional value. You define both.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>The update status of the capacity provider. The following are the possible states that is returned.</p> <dl> <dt>DELETE<em>IN</em>PROGRESS</dt> <dd> <p>The capacity provider is in the process of being deleted.</p> </dd> <dt>DELETE<em>COMPLETE</dt> <dd> <p>The capacity provider was successfully deleted and has an <code>INACTIVE</code> status.</p> </dd> <dt>DELETE</em>FAILED</dt> <dd> <p>The capacity provider can&#39;t be deleted. The update status reason provides further details about why the delete failed.</p> </dd> </dl></p>
    #[serde(rename = "updateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
    /// <p>The update status reason. This provides further details about the update status for the capacity provider.</p>
    #[serde(rename = "updateStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status_reason: Option<String>,
}

/// <p>The details of a capacity provider strategy. A capacity provider strategy can be set when using the <a>RunTask</a> or <a>CreateCluster</a> APIs or as the default capacity provider strategy for a cluster with the <a>CreateCluster</a> API.</p> <p>Only capacity providers that are already associated with a cluster and have an <code>ACTIVE</code> or <code>UPDATING</code> status can be used in a capacity provider strategy. The <a>PutClusterCapacityProviders</a> API is used to associate a capacity provider with a cluster.</p> <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must already be created. New Auto Scaling group capacity providers can be created with the <a>CreateCapacityProvider</a> API operation.</p> <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used in a capacity provider strategy.</p> <p>A capacity provider strategy may contain a maximum of 6 capacity providers.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CapacityProviderStrategyItem {
    /// <p>The <i>base</i> value designates how many tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a <i>base</i> defined. If no value is specified, the default value of <code>0</code> is used.</p>
    #[serde(rename = "base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i64>,
    /// <p>The short name of the capacity provider.</p>
    #[serde(rename = "capacityProvider")]
    pub capacity_provider: String,
    /// <p>The <i>weight</i> value designates the relative percentage of the total number of tasks launched that should use the specified capacity provider. The <code>weight</code> value is taken into consideration after the <code>base</code> value, if defined, is satisfied.</p> <p>If no <code>weight</code> value is specified, the default value of <code>0</code> is used. When multiple capacity providers are specified within a capacity provider strategy, at least one of the capacity providers must have a weight value greater than zero and any capacity providers with a weight of <code>0</code> can't be used to place tasks. If you specify multiple capacity providers in a strategy that all have a weight of <code>0</code>, any <code>RunTask</code> or <code>CreateService</code> actions using the capacity provider strategy will fail.</p> <p>An example scenario for using weights is defining a strategy that contains two capacity providers and both have a weight of <code>1</code>, then when the <code>base</code> is satisfied, the tasks will be split evenly across the two capacity providers. Using that same logic, if you specify a weight of <code>1</code> for <i>capacityProviderA</i> and a weight of <code>4</code> for <i>capacityProviderB</i>, then for every one task that's run using <i>capacityProviderA</i>, four tasks would use <i>capacityProviderB</i>.</p>
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// <p>A regional grouping of one or more container instances where you can run task requests. Each account receives a default cluster the first time you use the Amazon ECS service, but you may also create other clusters. Clusters may contain more than one instance type simultaneously.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Cluster {
    /// <p>The number of services that are running on the cluster in an <code>ACTIVE</code> state. You can view these services with <a>ListServices</a>.</p>
    #[serde(rename = "activeServicesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_services_count: Option<i64>,
    /// <p>The resources attached to a cluster. When using a capacity provider with a cluster, the capacity provider and associated resources are returned as cluster attachments.</p>
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// <p><p>The status of the capacity providers associated with the cluster. The following are the states that are returned.</p> <dl> <dt>UPDATE<em>IN</em>PROGRESS</dt> <dd> <p>The available capacity providers for the cluster are updating.</p> </dd> <dt>UPDATE<em>COMPLETE</dt> <dd> <p>The capacity providers have successfully updated.</p> </dd> <dt>UPDATE</em>FAILED</dt> <dd> <p>The capacity provider updates failed.</p> </dd> </dl></p>
    #[serde(rename = "attachmentsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments_status: Option<String>,
    /// <p>The capacity providers associated with the cluster.</p>
    #[serde(rename = "capacityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) that identifies the cluster. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>A user-generated string that you use to identify your cluster.</p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The execute command configuration for the cluster.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ClusterConfiguration>,
    /// <p>The default capacity provider strategy for the cluster. When services or tasks are run in the cluster with no launch type or capacity provider strategy specified, the default capacity provider strategy is used.</p>
    #[serde(rename = "defaultCapacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>The number of tasks in the cluster that are in the <code>PENDING</code> state.</p>
    #[serde(rename = "pendingTasksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_tasks_count: Option<i64>,
    /// <p>The number of container instances registered into the cluster. This includes container instances in both <code>ACTIVE</code> and <code>DRAINING</code> status.</p>
    #[serde(rename = "registeredContainerInstancesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_container_instances_count: Option<i64>,
    /// <p>The number of tasks in the cluster that are in the <code>RUNNING</code> state.</p>
    #[serde(rename = "runningTasksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i64>,
    /// <p>The settings for the cluster. This parameter indicates whether CloudWatch Container Insights is enabled or disabled for a cluster.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<ClusterSetting>>,
    /// <p><p>Additional information about your clusters that are separated by launch type. They include the following:</p> <ul> <li> <p>runningEC2TasksCount</p> </li> <li> <p>RunningFargateTasksCount</p> </li> <li> <p>pendingEC2TasksCount</p> </li> <li> <p>pendingFargateTasksCount</p> </li> <li> <p>activeEC2ServiceCount</p> </li> <li> <p>activeFargateServiceCount</p> </li> <li> <p>drainingEC2ServiceCount</p> </li> <li> <p>drainingFargateServiceCount</p> </li> </ul></p>
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Vec<KeyValuePair>>,
    /// <p><p>The status of the cluster. The following are the possible states that are returned.</p> <dl> <dt>ACTIVE</dt> <dd> <p>The cluster is ready to accept tasks and if applicable you can register container instances with the cluster.</p> </dd> <dt>PROVISIONING</dt> <dd> <p>The cluster has capacity providers that are associated with it and the resources needed for the capacity provider are being created.</p> </dd> <dt>DEPROVISIONING</dt> <dd> <p>The cluster has capacity providers that are associated with it and the resources needed for the capacity provider are being deleted.</p> </dd> <dt>FAILED</dt> <dd> <p>The cluster has capacity providers that are associated with it and the resources needed for the capacity provider have failed to create.</p> </dd> <dt>INACTIVE</dt> <dd> <p>The cluster has been deleted. Clusters with an <code>INACTIVE</code> status may remain discoverable in your account for a period of time. However, this behavior is subject to change in the future. We don&#39;t recommend that you rely on <code>INACTIVE</code> clusters persisting.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The metadata that you apply to the cluster to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The execute command configuration for the cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClusterConfiguration {
    /// <p>The details of the execute command configuration.</p>
    #[serde(rename = "executeCommandConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_command_configuration: Option<ExecuteCommandConfiguration>,
}

/// <p>The settings to use when creating a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClusterSetting {
    /// <p>The name of the cluster setting. The only supported value is <code>containerInsights</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value to set for the cluster setting. The supported values are <code>enabled</code> and <code>disabled</code>. If <code>enabled</code> is specified, CloudWatch Container Insights will be enabled for the cluster, otherwise it will be disabled unless the <code>containerInsights</code> account setting is enabled. If a cluster value is specified, it will override the <code>containerInsights</code> value set with <a>PutAccountSetting</a> or <a>PutAccountSettingDefault</a>.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A Docker container that's part of a task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Container {
    /// <p>The Amazon Resource Name (ARN) of the container.</p>
    #[serde(rename = "containerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arn: Option<String>,
    /// <p>The number of CPU units set for the container. The value is <code>0</code> if no value was specified in the container definition when the task definition was registered.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p>The exit code returned from the container.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The IDs of each GPU assigned to the container.</p>
    #[serde(rename = "gpuIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_ids: Option<Vec<String>>,
    /// <p>The health status of the container. If health checks aren't configured for this container in its task definition, then it reports the health status as <code>UNKNOWN</code>.</p>
    #[serde(rename = "healthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    /// <p>The image used for the container.</p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p><p>The container image manifest digest.</p> <note> <p>The <code>imageDigest</code> is only returned if the container is using an image hosted in Amazon ECR, otherwise it is omitted.</p> </note></p>
    #[serde(rename = "imageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    /// <p>The last known status of the container.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The details of any Amazon ECS managed agents associated with the container.</p>
    #[serde(rename = "managedAgents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_agents: Option<Vec<ManagedAgent>>,
    /// <p>The hard limit (in MiB) of memory set for the container.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>The soft limit (in MiB) of memory set for the container.</p>
    #[serde(rename = "memoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<String>,
    /// <p>The name of the container.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The network bindings associated with the container.</p>
    #[serde(rename = "networkBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    /// <p>The network interfaces associated with the container.</p>
    #[serde(rename = "networkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the Docker container.</p>
    #[serde(rename = "runtimeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,
    /// <p>The ARN of the task.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>Container definitions are used in task definitions to describe the different containers that are launched as part of a task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerDefinition {
    /// <p>The command that's passed to the container. This parameter maps to <code>Cmd</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>COMMAND</code> parameter to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>. For more information, see <a href="https://docs.docker.com/engine/reference/builder/#cmd">https://docs.docker.com/engine/reference/builder/#cmd</a>. If there are multiple arguments, each argument is a separated string in the array.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The number of <code>cpu</code> units reserved for the container. This parameter maps to <code>CpuShares</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--cpu-shares</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <p>This field is optional for tasks using the Fargate launch type, and the only requirement is that the total amount of CPU reserved for all containers within a task be lower than the task-level <code>cpu</code> value.</p> <note> <p>You can determine the number of CPU units that are available per EC2 instance type by multiplying the vCPUs listed for that instance type on the <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instances</a> detail page by 1,024.</p> </note> <p>Linux containers share unallocated CPU units with other containers on the container instance with the same ratio as their allocated amount. For example, if you run a single-container task on a single-core instance type with 512 CPU units specified for that container, and that's the only task running on the container instance, that container could use the full 1,024 CPU unit share at any given time. However, if you launched another copy of the same task on that container instance, each task is guaranteed a minimum of 512 CPU units when needed. Moreover, each container could float to higher CPU usage if the other container was not using it. If both tasks were 100% active all of the time, they would be limited to 512 CPU units.</p> <p>On Linux container instances, the Docker daemon on the container instance uses the CPU value to calculate the relative CPU share ratios for running containers. For more information, see <a href="https://docs.docker.com/engine/reference/run/#cpu-share-constraint">CPU share constraint</a> in the Docker documentation. The minimum valid CPU share value that the Linux kernel allows is 2. However, the CPU parameter isn't required, and you can use CPU values below 2 in your container definitions. For CPU values below 2 (including null), the behavior varies based on your Amazon ECS container agent version:</p> <ul> <li> <p> <b>Agent versions less than or equal to 1.1.0:</b> Null and zero CPU values are passed to Docker as 0, which Docker then converts to 1,024 CPU shares. CPU values of 1 are passed to Docker as 1, which the Linux kernel converts to two CPU shares.</p> </li> <li> <p> <b>Agent versions greater than or equal to 1.2.0:</b> Null, zero, and CPU values of 1 are passed to Docker as 2.</p> </li> </ul> <p>On Windows container instances, the CPU limit is enforced as an absolute limit, or a quota. Windows containers only have access to the specified amount of CPU that's described in the task definition. A null or zero CPU value is passed to Docker as <code>0</code>, which Windows interprets as 1% of one CPU.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i64>,
    /// <p><p>The dependencies defined for container startup and shutdown. A container can contain multiple dependencies. When a dependency is defined for container startup, for container shutdown it is reversed.</p> <p>For tasks using the EC2 launch type, the container instances require at least version 1.26.0 of the container agent to turn on container dependencies. However, we recommend using the latest container agent version. For information about checking your agent version and updating to the latest version, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html">Updating the Amazon ECS Container Agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. If you&#39;re using an Amazon ECS-optimized Linux AMI, your instance needs at least version 1.26.0-1 of the <code>ecs-init</code> package. If your container instances are launched from version <code>20190301</code> or later, then they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized Linux AMI</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>For tasks using the Fargate launch type, the task or service requires the following platforms:</p> <ul> <li> <p>Linux platform version <code>1.3.0</code> or later.</p> </li> <li> <p>Windows platform version <code>1.0.0</code> or later.</p> </li> </ul></p>
    #[serde(rename = "dependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<ContainerDependency>>,
    /// <p><p>When this parameter is true, networking is disabled within the container. This parameter maps to <code>NetworkDisabled</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "disableNetworking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_networking: Option<bool>,
    /// <p><p>A list of DNS search domains that are presented to the container. This parameter maps to <code>DnsSearch</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--dns-search</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "dnsSearchDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search_domains: Option<Vec<String>>,
    /// <p><p>A list of DNS servers that are presented to the container. This parameter maps to <code>Dns</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--dns</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "dnsServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    /// <p>A key/value map of labels to add to the container. This parameter maps to <code>Labels</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--label</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>. This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format '{{.Server.APIVersion}}'</code> </p>
    #[serde(rename = "dockerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of strings to provide custom labels for SELinux and AppArmor multi-level security systems. This field isn't valid for containers in tasks using the Fargate launch type.</p> <p>With Windows containers, this parameter can be used to reference a credential spec file when configuring a container for Active Directory authentication. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/windows-gmsa.html">Using gMSAs for Windows Containers</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>This parameter maps to <code>SecurityOpt</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--security-opt</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>The Amazon ECS container agent running on a container instance must register with the <code>ECS_SELINUX_CAPABLE=true</code> or <code>ECS_APPARMOR_CAPABLE=true</code> environment variables before containers placed on that instance can use these security options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note> <p>For more information about valid values, see <a href="https://docs.docker.com/engine/reference/run/#security-configuration">Docker Run Security Configuration</a>. </p> <p>Valid values: "no-new-privileges" | "apparmor:PROFILE" | "label:value" | "credentialspec:CredentialSpecFilePath"</p>
    #[serde(rename = "dockerSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_security_options: Option<Vec<String>>,
    /// <p><important> <p>Early versions of the Amazon ECS container agent don&#39;t properly handle <code>entryPoint</code> parameters. If you have problems using <code>entryPoint</code>, update your container agent or enter your commands and arguments as <code>command</code> array items instead.</p> </important> <p>The entry point that&#39;s passed to the container. This parameter maps to <code>Entrypoint</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--entrypoint</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>. For more information, see <a href="https://docs.docker.com/engine/reference/builder/#entrypoint">https://docs.docker.com/engine/reference/builder/#entrypoint</a>.</p></p>
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    /// <p><p>The environment variables to pass to a container. This parameter maps to <code>Env</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--env</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <important> <p>We don&#39;t recommend that you use plaintext environment variables for sensitive information, such as credential data.</p> </important></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>A list of files containing the environment variables to pass to a container. This parameter maps to the <code>--env-file</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <p>You can specify up to ten environment files. The file must have a <code>.env</code> file extension. Each line in an environment file contains an environment variable in <code>VARIABLE=VALUE</code> format. Lines beginning with <code>#</code> are treated as comments and are ignored. For more information about the environment variable file syntax, see <a href="https://docs.docker.com/compose/env-file/">Declare default environment variables in file</a>.</p> <p>If there are environment variables specified using the <code>environment</code> parameter in a container definition, they take precedence over the variables contained within an environment file. If multiple environment files are specified that contain the same variable, they're processed from the top down. We recommend that you use unique variable names. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/taskdef-envfiles.html">Specifying Environment Variables</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "environmentFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files: Option<Vec<EnvironmentFile>>,
    /// <p>If the <code>essential</code> parameter of a container is marked as <code>true</code>, and that container fails or stops for any reason, all other containers that are part of the task are stopped. If the <code>essential</code> parameter of a container is marked as <code>false</code>, its failure doesn't affect the rest of the containers in a task. If this parameter is omitted, a container is assumed to be essential.</p> <p>All tasks must have at least one essential container. If you have an application that's composed of multiple containers, group containers that are used for a common purpose into components, and separate the different components into multiple task definitions. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/application_architecture.html">Application Architecture</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "essential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    /// <p><p>A list of hostnames and IP address mappings to append to the <code>/etc/hosts</code> file on the container. This parameter maps to <code>ExtraHosts</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--add-host</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>This parameter isn&#39;t supported for Windows containers or tasks that use the <code>awsvpc</code> network mode.</p> </note></p>
    #[serde(rename = "extraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<HostEntry>>,
    /// <p>The FireLens configuration for the container. This is used to specify and configure a log router for container logs. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_firelens.html">Custom Log Routing</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "firelensConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firelens_configuration: Option<FirelensConfiguration>,
    /// <p>The container health check command and associated configuration parameters for the container. This parameter maps to <code>HealthCheck</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>HEALTHCHECK</code> parameter of <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
    #[serde(rename = "healthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    /// <p><p>The hostname to use for your container. This parameter maps to <code>Hostname</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--hostname</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>The <code>hostname</code> parameter is not supported if you&#39;re using the <code>awsvpc</code> network mode.</p> </note></p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p><p>The image used to start a container. This string is passed directly to the Docker daemon. By default, images in the Docker Hub registry are available. Other repositories are specified with either <code> <i>repository-url</i>/<i>image</i>:<i>tag</i> </code> or <code> <i>repository-url</i>/<i>image</i>@<i>digest</i> </code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>IMAGE</code> parameter of <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <ul> <li> <p>When a new task starts, the Amazon ECS container agent pulls the latest version of the specified image and tag for the container to use. However, subsequent updates to a repository image aren&#39;t propagated to already running tasks.</p> </li> <li> <p>Images in Amazon ECR repositories can be specified by either using the full <code>registry/repository:tag</code> or <code>registry/repository@digest</code>. For example, <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;:latest</code> or <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;@sha256:94afd1f2e64d908bc90dbca0035a5b567EXAMPLE</code>. </p> </li> <li> <p>Images in official repositories on Docker Hub use a single name (for example, <code>ubuntu</code> or <code>mongo</code>).</p> </li> <li> <p>Images in other repositories on Docker Hub are qualified with an organization name (for example, <code>amazon/amazon-ecs-agent</code>).</p> </li> <li> <p>Images in other online repositories are qualified further by a domain name (for example, <code>quay.io/assemblyline/ubuntu</code>).</p> </li> </ul></p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>When this parameter is <code>true</code>, you can deploy containerized applications that require <code>stdin</code> or a <code>tty</code> to be allocated. This parameter maps to <code>OpenStdin</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--interactive</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
    #[serde(rename = "interactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    /// <p><p>The <code>links</code> parameter allows containers to communicate with each other without the need for port mappings. This parameter is only supported if the network mode of a task definition is <code>bridge</code>. The <code>name:internalName</code> construct is analogous to <code>name:alias</code> in Docker links. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. For more information about linking Docker containers, go to <a href="https://docs.docker.com/network/links/">Legacy container links</a> in the Docker documentation. This parameter maps to <code>Links</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--link</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note> <important> <p>Containers that are collocated on a single container instance may be able to communicate with each other without requiring links or host port mappings. Network isolation is achieved on the container instance using security groups and VPC settings.</p> </important></p>
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// <p><p>Linux-specific modifications that are applied to the container, such as Linux kernel capabilities. For more information see <a>KernelCapabilities</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "linuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    /// <p><p>The log configuration specification for the container.</p> <p>This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>. By default, containers use the same logging driver that the Docker daemon uses. However the container can use a different logging driver than the Docker daemon by specifying a log driver with this parameter in the container definition. To use a different logging driver for a container, the log system must be configured properly on the container instance (or on a different log server for remote logging options). For more information about the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p> <note> <p>Amazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the <a>LogConfiguration</a> data type). Additional log drivers may be available in future releases of the Amazon ECS container agent.</p> </note> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format &#39;{{.Server.APIVersion}}&#39;</code> </p> <note> <p>The Amazon ECS container agent running on a container instance must register the logging drivers available on that instance with the <code>ECS<em>AVAILABLE</em>LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    #[serde(rename = "logConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    /// <p>The amount (in MiB) of memory to present to the container. If your container attempts to exceed the memory specified here, the container is killed. The total amount of memory reserved for all containers within a task must be lower than the task <code>memory</code> value, if one is specified. This parameter maps to <code>Memory</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--memory</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <p>If using the Fargate launch type, this parameter is optional.</p> <p>If using the EC2 launch type, you must specify either a task-level memory value or a container-level memory value. If you specify both a container-level <code>memory</code> and <code>memoryReservation</code> value, <code>memory</code> must be greater than <code>memoryReservation</code>. If you specify <code>memoryReservation</code>, then that value is subtracted from the available memory resources for the container instance where the container is placed. Otherwise, the value of <code>memory</code> is used.</p> <p>The Docker 20.10.0 or later daemon reserves a minimum of 6 MiB of memory for a container. So, don't specify less than 6 MiB of memory for your containers. </p> <p>The Docker 19.03.13-ce or earlier daemon reserves a minimum of 4 MiB of memory for a container. So, don't specify less than 4 MiB of memory for your containers.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The soft limit (in MiB) of memory to reserve for the container. When system memory is under heavy contention, Docker attempts to keep the container memory to this soft limit. However, your container can consume more memory when it needs to, up to either the hard limit specified with the <code>memory</code> parameter (if applicable), or all of the available memory on the container instance, whichever comes first. This parameter maps to <code>MemoryReservation</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--memory-reservation</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <p>If a task-level memory value is not specified, you must specify a non-zero integer for one or both of <code>memory</code> or <code>memoryReservation</code> in a container definition. If you specify both, <code>memory</code> must be greater than <code>memoryReservation</code>. If you specify <code>memoryReservation</code>, then that value is subtracted from the available memory resources for the container instance where the container is placed. Otherwise, the value of <code>memory</code> is used.</p> <p>For example, if your container normally uses 128 MiB of memory, but occasionally bursts to 256 MiB of memory for short periods of time, you can set a <code>memoryReservation</code> of 128 MiB, and a <code>memory</code> hard limit of 300 MiB. This configuration would allow the container to only reserve 128 MiB of memory from the remaining resources on the container instance, but also allow the container to consume more memory resources when needed.</p> <p>The Docker daemon reserves a minimum of 4 MiB of memory for a container. Therefore, we recommend that you specify fewer than 4 MiB of memory for your containers. </p>
    #[serde(rename = "memoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// <p>The mount points for data volumes in your container.</p> <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
    #[serde(rename = "mountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    /// <p>The name of a container. If you're linking multiple containers together in a task definition, the <code>name</code> of one container can be entered in the <code>links</code> of another container to connect the containers. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. This parameter maps to <code>name</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--name</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The list of port mappings for the container. Port mappings allow containers to access ports on the host container instance to send or receive traffic.</p> <p>For task definitions that use the <code>awsvpc</code> network mode, only specify the <code>containerPort</code>. The <code>hostPort</code> can be left blank or it must be the same value as the <code>containerPort</code>.</p> <p>Port mappings on Windows use the <code>NetNAT</code> gateway address rather than <code>localhost</code>. There&#39;s no loopback for port mappings on Windows, so you can&#39;t access a container&#39;s mapped port from the host itself. </p> <p>This parameter maps to <code>PortBindings</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--publish</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>. If the network mode of a task definition is set to <code>none</code>, then you can&#39;t specify port mappings. If the network mode of a task definition is set to <code>host</code>, then host ports must either be undefined or they must match the container port in the port mapping.</p> <note> <p>After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <b>Network Bindings</b> section of a container description for a selected task in the Amazon ECS console. The assignments are also visible in the <code>networkBindings</code> section <a>DescribeTasks</a> responses.</p> </note></p>
    #[serde(rename = "portMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_mappings: Option<Vec<PortMapping>>,
    /// <p><p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user). This parameter maps to <code>Privileged</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--privileged</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers or tasks run on Fargate.</p> </note></p>
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>When this parameter is <code>true</code>, a TTY is allocated. This parameter maps to <code>Tty</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--tty</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
    #[serde(rename = "pseudoTerminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudo_terminal: Option<bool>,
    /// <p><p>When this parameter is true, the container is given read-only access to its root file system. This parameter maps to <code>ReadonlyRootfs</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--read-only</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>The private repository authentication credentials to use.</p>
    #[serde(rename = "repositoryCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,
    /// <p>The type and amount of a resource to assign to a container. The only supported resource is a GPU.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    /// <p>The secrets to pass to the container. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/specifying-sensitive-data.html">Specifying Sensitive Data</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    /// <p>Time duration (in seconds) to wait before giving up on resolving dependencies for a container. For example, you specify two containers in a task definition with containerA having a dependency on containerB reaching a <code>COMPLETE</code>, <code>SUCCESS</code>, or <code>HEALTHY</code> status. If a <code>startTimeout</code> value is specified for containerB and it doesn't reach the desired status within that time then containerA gives up and not start. This results in the task transitioning to a <code>STOPPED</code> state.</p> <note> <p>When the <code>ECS_CONTAINER_START_TIMEOUT</code> container agent configuration variable is used, it's enforced independently from this start timeout value.</p> </note> <p>For tasks using the Fargate launch type, the task or service requires the following platforms:</p> <ul> <li> <p>Linux platform version <code>1.3.0</code> or later.</p> </li> <li> <p>Windows platform version <code>1.0.0</code> or later.</p> </li> </ul> <p>For tasks using the EC2 launch type, your container instances require at least version <code>1.26.0</code> of the container agent to use a container start timeout value. However, we recommend using the latest container agent version. For information about checking your agent version and updating to the latest version, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html">Updating the Amazon ECS Container Agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. If you're using an Amazon ECS-optimized Linux AMI, your instance needs at least version <code>1.26.0-1</code> of the <code>ecs-init</code> package. If your container instances are launched from version <code>20190301</code> or later, then they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized Linux AMI</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "startTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timeout: Option<i64>,
    /// <p>Time duration (in seconds) to wait before the container is forcefully killed if it doesn't exit normally on its own.</p> <p>For tasks using the Fargate launch type, the task or service requires the following platforms:</p> <ul> <li> <p>Linux platform version <code>1.3.0</code> or later.</p> </li> <li> <p>Windows platform version <code>1.0.0</code> or later.</p> </li> </ul> <p>The max stop timeout value is 120 seconds and if the parameter is not specified, the default value of 30 seconds is used.</p> <p>For tasks that use the EC2 launch type, if the <code>stopTimeout</code> parameter isn't specified, the value set for the Amazon ECS container agent configuration variable <code>ECS_CONTAINER_STOP_TIMEOUT</code> is used. If neither the <code>stopTimeout</code> parameter or the <code>ECS_CONTAINER_STOP_TIMEOUT</code> agent configuration variable are set, then the default values of 30 seconds for Linux containers and 30 seconds on Windows containers are used. Your container instances require at least version 1.26.0 of the container agent to use a container stop timeout value. However, we recommend using the latest container agent version. For information about checking your agent version and updating to the latest version, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html">Updating the Amazon ECS Container Agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. If you're using an Amazon ECS-optimized Linux AMI, your instance needs at least version 1.26.0-1 of the <code>ecs-init</code> package. If your container instances are launched from version <code>20190301</code> or later, then they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized Linux AMI</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "stopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    /// <p><p>A list of namespaced kernel parameters to set in the container. This parameter maps to <code>Sysctls</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--sysctl</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>We don&#39;t recommended that you specify network-related <code>systemControls</code> parameters for multiple containers in a single task that also uses either the <code>awsvpc</code> or <code>host</code> network modes. For tasks that use the <code>awsvpc</code> network mode, the container that&#39;s started last determines which <code>systemControls</code> parameters take effect. For tasks that use the <code>host</code> network mode, it changes the container instance&#39;s namespaced kernel parameters as well as the containers.</p> </note></p>
    #[serde(rename = "systemControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_controls: Option<Vec<SystemControl>>,
    /// <p><p>A list of <code>ulimits</code> to set in the container. If a ulimit value is specified in a task definition, it overrides the default values set by Docker. This parameter maps to <code>Ulimits</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--ulimit</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>. Valid naming values are displayed in the <a>Ulimit</a> data type.</p> <p>Amazon ECS tasks hosted on Fargate use the default resource limit values set by the operating system with the exception of the <code>nofile</code> resource limit parameter which Fargate overrides. The <code>nofile</code> resource limit sets a restriction on the number of open files that a container can use. The default <code>nofile</code> soft limit is <code>1024</code> and hard limit is <code>4096</code>.</p> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format &#39;{{.Server.APIVersion}}&#39;</code> </p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    /// <p><p>The user to use inside the container. This parameter maps to <code>User</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--user</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <important> <p>When running tasks using the <code>host</code> network mode, don&#39;t run containers using the root user (UID 0). We recommend using a non-root user for better security.</p> </important> <p>You can specify the <code>user</code> using the following formats. If specifying a UID or GID, you must specify it as a positive integer.</p> <ul> <li> <p> <code>user</code> </p> </li> <li> <p> <code>user:group</code> </p> </li> <li> <p> <code>uid</code> </p> </li> <li> <p> <code>uid:gid</code> </p> </li> <li> <p> <code>user:gid</code> </p> </li> <li> <p> <code>uid:group</code> </p> </li> </ul> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p>Data volumes to mount from another container. This parameter maps to <code>VolumesFrom</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--volumes-from</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
    #[serde(rename = "volumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<VolumeFrom>>,
    /// <p>The working directory to run commands inside the container in. This parameter maps to <code>WorkingDir</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--workdir</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
    #[serde(rename = "workingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// <p><p>The dependencies defined for container startup and shutdown. A container can contain multiple dependencies. When a dependency is defined for container startup, for container shutdown it is reversed.</p> <p>Your Amazon ECS container instances require at least version 1.26.0 of the container agent to use container dependencies. However, we recommend using the latest container agent version. For information about checking your agent version and updating to the latest version, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html">Updating the Amazon ECS Container Agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. If you&#39;re using an Amazon ECS-optimized Linux AMI, your instance needs at least version 1.26.0-1 of the <code>ecs-init</code> package. If your container instances are launched from version <code>20190301</code> or later, then they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized Linux AMI</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>For tasks that use the Fargate launch type, the task or service requires the following platforms:</p> <ul> <li> <p>Linux platform version <code>1.3.0</code> or later.</p> </li> <li> <p>Windows platform version <code>1.0.0</code> or later.</p> </li> </ul> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerDependency {
    /// <p><p>The dependency condition of the container. The following are the available conditions and their behavior:</p> <ul> <li> <p> <code>START</code> - This condition emulates the behavior of links and volumes today. It validates that a dependent container is started before permitting other containers to start.</p> </li> <li> <p> <code>COMPLETE</code> - This condition validates that a dependent container runs to completion (exits) before permitting other containers to start. This can be useful for nonessential containers that run a script and then exit. This condition can&#39;t be set on an essential container.</p> </li> <li> <p> <code>SUCCESS</code> - This condition is the same as <code>COMPLETE</code>, but it also requires that the container exits with a <code>zero</code> status. This condition can&#39;t be set on an essential container.</p> </li> <li> <p> <code>HEALTHY</code> - This condition validates that the dependent container passes its Docker health check before permitting other containers to start. This requires that the dependent container has health checks configured. This condition is confirmed only at task startup.</p> </li> </ul></p>
    #[serde(rename = "condition")]
    pub condition: String,
    /// <p>The name of a container.</p>
    #[serde(rename = "containerName")]
    pub container_name: String,
}

/// <p>An Amazon EC2 or External instance that's running the Amazon ECS agent and has been registered with a cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContainerInstance {
    /// <p>This parameter returns <code>true</code> if the agent is connected to Amazon ECS. An instance with an agent that may be unhealthy or stopped return <code>false</code>. Only instances connected to an agent can accept task placement requests.</p>
    #[serde(rename = "agentConnected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_connected: Option<bool>,
    /// <p>The status of the most recent agent update. If an update wasn't ever requested, this value is <code>NULL</code>.</p>
    #[serde(rename = "agentUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_update_status: Option<String>,
    /// <p>The resources attached to a container instance, such as an elastic network interface.</p>
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// <p>The attributes set for the container instance, either by the Amazon ECS container agent at instance registration or manually with the <a>PutAttributes</a> operation.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The capacity provider that's associated with the container instance.</p>
    #[serde(rename = "capacityProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the container instance. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The ID of the container instance. For Amazon EC2 instances, this value is the Amazon EC2 instance ID. For external instances, this value is the Amazon Web Services Systems Manager managed instance ID.</p>
    #[serde(rename = "ec2InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    /// <p>An object representing the health status of the container instance.</p>
    #[serde(rename = "healthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<ContainerInstanceHealthStatus>,
    /// <p>The number of tasks on the container instance that are in the <code>PENDING</code> status.</p>
    #[serde(rename = "pendingTasksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_tasks_count: Option<i64>,
    /// <p>The Unix timestamp for the time when the container instance was registered.</p>
    #[serde(rename = "registeredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    /// <p>For CPU and memory resource types, this parameter describes the amount of each resource that was available on the container instance when the container agent registered it with Amazon ECS. This value represents the total amount of CPU and memory that can be allocated on this container instance to tasks. For port resource types, this parameter describes the ports that were reserved by the Amazon ECS container agent when it registered the container instance with Amazon ECS.</p>
    #[serde(rename = "registeredResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_resources: Option<Vec<Resource>>,
    /// <p>For CPU and memory resource types, this parameter describes the remaining CPU and memory that wasn't already allocated to tasks and is therefore available for new tasks. For port resource types, this parameter describes the ports that were reserved by the Amazon ECS container agent (at instance registration time) and any task containers that have reserved port mappings on the host (with the <code>host</code> or <code>bridge</code> network mode). Any port that's not specified here is available for new tasks.</p>
    #[serde(rename = "remainingResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_resources: Option<Vec<Resource>>,
    /// <p>The number of tasks on the container instance that are in the <code>RUNNING</code> status.</p>
    #[serde(rename = "runningTasksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i64>,
    /// <p>The status of the container instance. The valid values are <code>REGISTERING</code>, <code>REGISTRATION_FAILED</code>, <code>ACTIVE</code>, <code>INACTIVE</code>, <code>DEREGISTERING</code>, or <code>DRAINING</code>.</p> <p>If your account has opted in to the <code>awsvpcTrunking</code> account setting, then any newly registered container instance will transition to a <code>REGISTERING</code> status while the trunk elastic network interface is provisioned for the instance. If the registration fails, the instance will transition to a <code>REGISTRATION_FAILED</code> status. You can describe the container instance and see the reason for failure in the <code>statusReason</code> parameter. Once the container instance is terminated, the instance transitions to a <code>DEREGISTERING</code> status while the trunk elastic network interface is deprovisioned. The instance then transitions to an <code>INACTIVE</code> status.</p> <p>The <code>ACTIVE</code> status indicates that the container instance can accept tasks. The <code>DRAINING</code> indicates that new tasks aren't placed on the container instance and any service tasks running on the container instance are removed if possible. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-draining.html">Container instance draining</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The reason that the container instance reached its current status.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p><p>The metadata that you apply to the container instance to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The version counter for the container instance. Every time a container instance experiences a change that triggers a CloudWatch event, the version counter is incremented. If you're replicating your Amazon ECS container instance state with CloudWatch Events, you can compare the version of a container instance reported by the Amazon ECS APIs with the version reported in CloudWatch Events for the container instance (inside the <code>detail</code> object) to verify that the version in your event stream is current.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// <p>The version information for the Amazon ECS container agent and Docker daemon running on the container instance.</p>
    #[serde(rename = "versionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<VersionInfo>,
}

/// <p>An object representing the health status of the container instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContainerInstanceHealthStatus {
    /// <p>An array of objects representing the details of the container instance health status.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<InstanceHealthCheckResult>>,
    /// <p>The overall health status of the container instance. This is an aggregate status of all container instance health checks.</p>
    #[serde(rename = "overallStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_status: Option<String>,
}

/// <p>The overrides that are sent to a container. An empty container override can be passed in. An example of an empty container override is <code>{"containerOverrides": [ ] }</code>. If a non-empty container override is specified, the <code>name</code> parameter must be included.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerOverride {
    /// <p>The command to send to the container that overrides the default command from the Docker image or the task definition. You must also specify a container name.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The number of <code>cpu</code> units reserved for the container, instead of the default value from the task definition. You must also specify a container name.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i64>,
    /// <p>The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the task definition. You must also specify a container name.</p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>A list of files containing the environment variables to pass to a container, instead of the value from the container definition.</p>
    #[serde(rename = "environmentFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files: Option<Vec<EnvironmentFile>>,
    /// <p>The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition. If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The soft limit (in MiB) of memory to reserve for the container, instead of the default value from the task definition. You must also specify a container name.</p>
    #[serde(rename = "memoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// <p>The name of the container that receives the override. This parameter is required if any override is specified.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type and amount of a resource to assign to a container, instead of the default value from the task definition. The only supported resource is a GPU.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
}

/// <p>An object that represents a change in state for a container.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContainerStateChange {
    /// <p>The name of the container.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The exit code for the container, if the state change is a result of the container exiting.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The container image SHA 256 digest.</p>
    #[serde(rename = "imageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    /// <p>Any network bindings that are associated with the container.</p>
    #[serde(rename = "networkBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    /// <p>The reason for the state change.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the Docker container.</p>
    #[serde(rename = "runtimeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,
    /// <p>The status of the container.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCapacityProviderRequest {
    /// <p>The details of the Auto Scaling group for the capacity provider.</p>
    #[serde(rename = "autoScalingGroupProvider")]
    pub auto_scaling_group_provider: AutoScalingGroupProvider,
    /// <p>The name of the capacity provider. Up to 255 characters are allowed. They include letters (both upper and lowercase letters), numbers, underscores (_), and hyphens (-). The name can't be prefixed with "<code>aws</code>", "<code>ecs</code>", or "<code>fargate</code>".</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The metadata that you apply to the capacity provider to categorize and organize them more conveniently. Each tag consists of a key and an optional value. You define both of them.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCapacityProviderResponse {
    /// <p>The full description of the new capacity provider.</p>
    #[serde(rename = "capacityProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateClusterRequest {
    /// <p>The short name of one or more capacity providers to associate with the cluster. A capacity provider must be associated with a cluster before it can be included as part of the default capacity provider strategy of the cluster or used in a capacity provider strategy when calling the <a>CreateService</a> or <a>RunTask</a> actions.</p> <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must be created but not associated with another cluster. New Auto Scaling group capacity providers can be created with the <a>CreateCapacityProvider</a> API operation.</p> <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p> <p>The <a>PutClusterCapacityProviders</a> API operation is used to update the list of available capacity providers for a cluster after the cluster is created.</p>
    #[serde(rename = "capacityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<String>>,
    /// <p>The name of your cluster. If you don't specify a name for your cluster, you create a cluster that's named <code>default</code>. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. </p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The <code>execute</code> command configuration for the cluster.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ClusterConfiguration>,
    /// <p>The capacity provider strategy to set as the default for the cluster. After a default capacity provider strategy is set for a cluster, when you call the <a>RunTask</a> or <a>CreateService</a> APIs with no capacity provider strategy or launch type specified, the default capacity provider strategy for the cluster is used.</p> <p>If a default capacity provider strategy isn't defined for a cluster when it was created, it can be defined later with the <a>PutClusterCapacityProviders</a> API operation.</p>
    #[serde(rename = "defaultCapacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>The setting to use when creating a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <a>PutAccountSetting</a> or <a>PutAccountSettingDefault</a>.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<ClusterSetting>>,
    /// <p><p>The metadata that you apply to the cluster to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>The full description of your new cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateServiceRequest {
    /// <p>The capacity provider strategy to use for the service.</p> <p>If a <code>capacityProviderStrategy</code> is specified, the <code>launchType</code> parameter must be omitted. If no <code>capacityProviderStrategy</code> or <code>launchType</code> is specified, the <code>defaultCapacityProviderStrategy</code> for the cluster is used.</p> <p>A capacity provider strategy may contain a maximum of 6 capacity providers.</p>
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>An identifier that you provide to ensure the idempotency of the request. It must be unique and is case sensitive. Up to 32 ASCII characters are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that you run your service on. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>
    #[serde(rename = "deploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    /// <p>The deployment controller to use for the service. If no deployment controller is specified, the default value of <code>ECS</code> is used.</p>
    #[serde(rename = "deploymentController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<DeploymentController>,
    /// <p>The number of instantiations of the specified task definition to place and keep running on your cluster.</p> <p>This is required if <code>schedulingStrategy</code> is <code>REPLICA</code> or isn't specified. If <code>schedulingStrategy</code> is <code>DAEMON</code> then this isn't required.</p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p>Specifies whether to turn on Amazon ECS managed tags for the tasks within the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging your Amazon ECS resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>Determines whether the execute command functionality is enabled for the service. If <code>true</code>, this enables execute command functionality on all containers in the service tasks.</p>
    #[serde(rename = "enableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    /// <p>The period of time, in seconds, that the Amazon ECS service scheduler ignores unhealthy Elastic Load Balancing target health checks after a task has first started. This is only used when your service is configured to use a load balancer. If your service has a load balancer defined and you don't specify a health check grace period value, the default value of <code>0</code> is used.</p> <p>If you do not use an Elastic Load Balancing, we recomend that you use the <code>startPeriod</code> in the task definition healtch check parameters. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_HealthCheck.html">Health check</a>.</p> <p>If your service's tasks take a while to start and respond to Elastic Load Balancing health checks, you can specify a health check grace period of up to 2,147,483,647 seconds (about 69 years). During that time, the Amazon ECS service scheduler ignores health check status. This grace period can prevent the service scheduler from marking tasks as unhealthy and stopping them before they have time to come up.</p>
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i64>,
    /// <p>The infrastructure that you run your service on. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS launch types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>The <code>FARGATE</code> launch type runs your tasks on Fargate On-Demand infrastructure.</p> <note> <p>Fargate Spot infrastructure is available for use but a capacity provider strategy must be used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/userguide/fargate-capacity-providers.html">Fargate capacity providers</a> in the <i>Amazon ECS User Guide for Fargate</i>.</p> </note> <p>The <code>EC2</code> launch type runs your tasks on Amazon EC2 instances registered to your cluster.</p> <p>The <code>EXTERNAL</code> launch type runs your tasks on your on-premises server or virtual machine (VM) capacity registered to your cluster.</p> <p>A service can use either a launch type or a capacity provider strategy. If a <code>launchType</code> is specified, the <code>capacityProviderStrategy</code> parameter must be omitted.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>A load balancer object representing the load balancers to use with your service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-load-balancing.html">Service load balancing</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>If the service uses the rolling update (<code>ECS</code>) deployment controller and using either an Application Load Balancer or Network Load Balancer, you must specify one or more target group ARNs to attach to the service. The service-linked role is required for services that use multiple target groups. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using service-linked roles for Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>If the service uses the <code>CODE_DEPLOY</code> deployment controller, the service is required to use either an Application Load Balancer or Network Load Balancer. When creating an CodeDeploy deployment group, you specify two target groups (referred to as a <code>targetGroupPair</code>). During a deployment, CodeDeploy determines which task set in your service has the status <code>PRIMARY</code>, and it associates one target group with it. Then, it also associates the other target group with the replacement task set. The load balancer can also have up to two listeners: a required listener for production traffic and an optional listener that you can use to perform validation tests with Lambda functions before routing production traffic to it.</p> <p>If you use the <code>CODE_DEPLOY</code> deployment controller, these values can be changed when updating the service.</p> <p>For Application Load Balancers and Network Load Balancers, this object must contain the load balancer target group ARN, the container name, and the container port to access from the load balancer. The container name must be as it appears in a container definition. The load balancer name parameter must be omitted. When a task from this service is placed on a container instance, the container instance and port combination is registered as a target in the target group that's specified here.</p> <p>For Classic Load Balancers, this object must contain the load balancer name, the container name , and the container port to access from the load balancer. The container name must be as it appears in a container definition. The target group ARN parameter must be omitted. When a task from this service is placed on a container instance, the container instance is registered with the load balancer that's specified here.</p> <p>Services with tasks that use the <code>awsvpc</code> network mode (for example, those with the Fargate launch type) only support Application Load Balancers and Network Load Balancers. Classic Load Balancers aren't supported. Also, when you create any target groups for these services, you must choose <code>ip</code> as the target type, not <code>instance</code>. This is because tasks that use the <code>awsvpc</code> network mode are associated with an elastic network interface, not an Amazon EC2 instance.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>The network configuration for the service. This parameter is required for task definitions that use the <code>awsvpc</code> network mode to receive their own elastic network interface, and it isn't supported for other network modes. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>An array of placement constraint objects to use for tasks in your service. You can specify a maximum of 10 constraints for each task. This limit includes constraints in the task definition and those specified at runtime.</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// <p>The placement strategy objects to use for tasks in your service. You can specify a maximum of 5 strategy rules for each service.</p>
    #[serde(rename = "placementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    /// <p>The platform version that your tasks in the service are running on. A platform version is specified only for tasks using the Fargate launch type. If one isn't specified, the <code>LATEST</code> platform version is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate platform versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags aren't propagated. Tags can only be propagated to the task during task creation. To add tags to a task after task creation, use the <a>TagResource</a> API action.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>The name or full Amazon Resource Name (ARN) of the IAM role that allows Amazon ECS to make calls to your load balancer on your behalf. This parameter is only permitted if you are using a load balancer with your service and your task definition doesn't use the <code>awsvpc</code> network mode. If you specify the <code>role</code> parameter, you must also specify a load balancer object with the <code>loadBalancers</code> parameter.</p> <important> <p>If your account has already created the Amazon ECS service-linked role, that role is used for your service unless you specify a role here. The service-linked role is required if your task definition uses the <code>awsvpc</code> network mode or if the service is configured to use service discovery, an external deployment controller, multiple target groups, or Elastic Inference accelerators in which case you don't specify a role here. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using service-linked roles for Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </important> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path. For example, if a role with the name <code>bar</code> has a path of <code>/foo/</code> then you would specify <code>/foo/bar</code> as the role name. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-friendly-names">Friendly names and paths</a> in the <i>IAM User Guide</i>.</p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p><p>The scheduling strategy to use for the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Services</a>.</p> <p>There are two service scheduler strategies available:</p> <ul> <li> <p> <code>REPLICA</code>-The replica scheduling strategy places and maintains the desired number of tasks across your cluster. By default, the service scheduler spreads tasks across Availability Zones. You can use task placement strategies and constraints to customize task placement decisions. This scheduler strategy is required if the service uses the <code>CODE<em>DEPLOY</code> or <code>EXTERNAL</code> deployment controller types.</p> </li> <li> <p> <code>DAEMON</code>-The daemon scheduling strategy deploys exactly one task on each active container instance that meets all of the task placement constraints that you specify in your cluster. The service scheduler also evaluates the task placement constraints for running tasks and will stop tasks that don&#39;t meet the placement constraints. When you&#39;re using this strategy, you don&#39;t need to specify a desired number of tasks, a task placement strategy, or use Service Auto Scaling policies.</p> <note> <p>Tasks using the Fargate launch type or the <code>CODE</em>DEPLOY</code> or <code>EXTERNAL</code> deployment controller types don&#39;t support the <code>DAEMON</code> scheduling strategy.</p> </note> </li> </ul></p>
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// <p>The name of your service. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. Service names must be unique within a cluster, but you can have similarly named services in multiple clusters within a Region or across multiple Regions.</p>
    #[serde(rename = "serviceName")]
    pub service_name: String,
    /// <p><p>The details of the service discovery registry to associate with this service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html">Service discovery</a>.</p> <note> <p>Each service may be associated with one service registry. Multiple service registries for each service isn&#39;t supported.</p> </note></p>
    #[serde(rename = "serviceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// <p><p>The metadata that you apply to the service to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. When a service is deleted, the tags are deleted as well.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full ARN of the task definition to run in your service. If a <code>revision</code> isn't specified, the latest <code>ACTIVE</code> revision is used.</p> <p>A task definition must be specified if the service uses either the <code>ECS</code> or <code>CODE_DEPLOY</code> deployment controllers.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateServiceResponse {
    /// <p>The full description of your service following the create call.</p> <p>A service will return either a <code>capacityProviderStrategy</code> or <code>launchType</code> parameter, but not both, depending where one was specified when it was created.</p> <p>If a service is using the <code>ECS</code> deployment controller, the <code>deploymentController</code> and <code>taskSets</code> parameters will not be returned.</p> <p>if the service uses the <code>CODE_DEPLOY</code> deployment controller, the <code>deploymentController</code>, <code>taskSets</code> and <code>deployments</code> parameters will be returned, however the <code>deployments</code> parameter will be an empty list.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTaskSetRequest {
    /// <p>The capacity provider strategy to use for the task set.</p> <p>A capacity provider strategy consists of one or more capacity providers along with the <code>base</code> and <code>weight</code> to assign to them. A capacity provider must be associated with the cluster to be used in a capacity provider strategy. The <a>PutClusterCapacityProviders</a> API is used to associate a capacity provider with a cluster. Only capacity providers with an <code>ACTIVE</code> or <code>UPDATING</code> status can be used.</p> <p>If a <code>capacityProviderStrategy</code> is specified, the <code>launchType</code> parameter must be omitted. If no <code>capacityProviderStrategy</code> or <code>launchType</code> is specified, the <code>defaultCapacityProviderStrategy</code> for the cluster is used.</p> <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must already be created. New capacity providers can be created with the <a>CreateCapacityProvider</a> API operation.</p> <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p> <p>The <a>PutClusterCapacityProviders</a> API operation is used to update the list of available capacity providers for a cluster after the cluster is created.</p>
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>The identifier that you provide to ensure the idempotency of the request. It's case sensitive and must be unique. It can be up to 32 ASCII characters are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service to create the task set in.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// <p>An optional non-unique tag that identifies this task set in external systems. If the task set is associated with a service discovery registry, the tasks in this task set will have the <code>ECS_TASK_SET_EXTERNAL_ID</code> Cloud Map attribute set to the provided value.</p>
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The launch type that new tasks in the task set uses. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS launch types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>If a <code>launchType</code> is specified, the <code>capacityProviderStrategy</code> parameter must be omitted.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>A load balancer object representing the load balancer to use with the task set. The supported load balancer types are either an Application Load Balancer or a Network Load Balancer.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>An object representing the network configuration for a task set.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The platform version that the tasks in the task set uses. A platform version is specified only for tasks using the Fargate launch type. If one isn't specified, the <code>LATEST</code> platform version is used.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>A floating-point percentage of the desired number of tasks to place and keep running in the task set.</p>
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the service to create the task set in.</p>
    #[serde(rename = "service")]
    pub service: String,
    /// <p>The details of the service discovery registries to assign to this task set. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html">Service discovery</a>.</p>
    #[serde(rename = "serviceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// <p><p>The metadata that you apply to the task set to help you categorize and organize them. Each tag consists of a key and an optional value. You define both. When a service is deleted, the tags are deleted.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The task definition for the tasks in the task set to use.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTaskSetResponse {
    /// <p>Information about a set of Amazon ECS tasks in either an CodeDeploy or an <code>EXTERNAL</code> deployment. A task set includes details such as the desired number of tasks, how many tasks are running, and whether the task set serves production traffic.</p>
    #[serde(rename = "taskSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set: Option<TaskSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAccountSettingRequest {
    /// <p>The resource name to disable the account setting for. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the ENI limit for your Amazon ECS container instances is affected.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the principal. It can be an IAM user, IAM role, or the root user. If you specify the root user, it disables the account setting for all IAM users, IAM roles, and the root user of the account unless an IAM user or role explicitly overrides these settings. If this field is omitted, the setting is changed only for the authenticated user.</p>
    #[serde(rename = "principalArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAccountSettingResponse {
    /// <p>The account setting for the specified principal ARN.</p>
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAttributesRequest {
    /// <p>The attributes to delete from your resource. You can specify up to 10 attributes for each request. For custom attributes, specify the attribute name and target ID, but don't specify the value. If you specify the target ID using the short form, you must also specify the target type.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to delete attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAttributesResponse {
    /// <p>A list of attribute objects that were successfully deleted from your resource.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCapacityProviderRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the capacity provider to delete.</p>
    #[serde(rename = "capacityProvider")]
    pub capacity_provider: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCapacityProviderResponse {
    /// <p>The details of the capacity provider.</p>
    #[serde(rename = "capacityProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteClusterRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to delete.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>The full description of the deleted cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteServiceRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service to delete. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>If <code>true</code>, allows you to delete a service even if it wasn't scaled down to zero tasks. It's only necessary to use this if the service uses the <code>REPLICA</code> scheduling strategy.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The name of the service to delete.</p>
    #[serde(rename = "service")]
    pub service: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteServiceResponse {
    /// <p>The full description of the deleted service.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTaskSetRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set found in to delete.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// <p>If <code>true</code>, you can delete a task set even if it hasn't been scaled down to zero.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the service that hosts the task set to delete.</p>
    #[serde(rename = "service")]
    pub service: String,
    /// <p>The task set ID or full Amazon Resource Name (ARN) of the task set to delete.</p>
    #[serde(rename = "taskSet")]
    pub task_set: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTaskSetResponse {
    /// <p>Details about the task set.</p>
    #[serde(rename = "taskSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set: Option<TaskSet>,
}

/// <p>The details of an Amazon ECS service deployment. This is used only when a service uses the <code>ECS</code> deployment controller type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Deployment {
    /// <p>The capacity provider strategy that the deployment is using.</p>
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>The Unix timestamp for the time when the service deployment was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The most recent desired count of tasks that was specified for the service to deploy or maintain.</p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p><p>The number of consecutively failed tasks in the deployment. A task is considered a failure if the service scheduler can&#39;t launch the task, the task doesn&#39;t transition to a <code>RUNNING</code> state, or if it fails any of its defined health checks and is stopped.</p> <note> <p>Once a service deployment has one or more successfully running tasks, the failed task count resets to zero and stops being evaluated.</p> </note></p>
    #[serde(rename = "failedTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_tasks: Option<i64>,
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The launch type the tasks in the service are using. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS Launch Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>The VPC subnet and security group configuration for tasks that receive their own elastic network interface by using the <code>awsvpc</code> networking mode.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The number of tasks in the deployment that are in the <code>PENDING</code> status.</p>
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i64>,
    /// <p>The operating system that your tasks in the service, or tasks are running on. A platform family is specified only for tasks using the Fargate launch type. </p> <p> All tasks that run as part of this service must use the same <code>platformFamily</code> value as the service, for example, <code> LINUX.</code>.</p>
    #[serde(rename = "platformFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    /// <p>The platform version that your tasks in the service run on. A platform version is only specified for tasks using the Fargate launch type. If one isn't specified, the <code>LATEST</code> platform version is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p><note> <p>The <code>rolloutState</code> of a service is only returned for services that use the rolling update (<code>ECS</code>) deployment type that aren&#39;t behind a Classic Load Balancer.</p> </note> <p>The rollout state of the deployment. When a service deployment is started, it begins in an <code>IN_PROGRESS</code> state. When the service reaches a steady state, the deployment transitions to a <code>COMPLETED</code> state. If the service fails to reach a steady state and circuit breaker is enabled, the deployment transitions to a <code>FAILED</code> state. A deployment in <code>FAILED</code> state doesn&#39;t launch any new tasks. For more information, see <a>DeploymentCircuitBreaker</a>.</p></p>
    #[serde(rename = "rolloutState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_state: Option<String>,
    /// <p>A description of the rollout state of a deployment.</p>
    #[serde(rename = "rolloutStateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_state_reason: Option<String>,
    /// <p>The number of tasks in the deployment that are in the <code>RUNNING</code> status.</p>
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i64>,
    /// <p><p>The status of the deployment. The following describes each state.</p> <dl> <dt>PRIMARY</dt> <dd> <p>The most recent deployment of a service.</p> </dd> <dt>ACTIVE</dt> <dd> <p>A service deployment that still has running tasks, but are in the process of being replaced with a new <code>PRIMARY</code> deployment.</p> </dd> <dt>INACTIVE</dt> <dd> <p>A deployment that has been completely replaced.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The most recent task definition that was specified for the tasks in the service to use.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    /// <p>The Unix timestamp for the time when the service deployment was last updated.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p><note> <p>The deployment circuit breaker can only be used for services using the rolling update (<code>ECS</code>) deployment type that aren&#39;t behind a Classic Load Balancer.</p> </note> <p>The <b>deployment circuit breaker</b> determines whether a service deployment will fail if the service can&#39;t reach a steady state. If enabled, a service deployment will transition to a failed state and stop launching new tasks. You can also configure Amazon ECS to roll back your service to the last completed deployment after a failure. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-type-ecs.html">Rolling update</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeploymentCircuitBreaker {
    /// <p>Determines whether to use the deployment circuit breaker logic for the service.</p>
    #[serde(rename = "enable")]
    pub enable: bool,
    /// <p>Determines whether to configure Amazon ECS to roll back the service if a service deployment fails. If rollback is enabled, when a service deployment fails, the service is rolled back to the last deployment that completed successfully.</p>
    #[serde(rename = "rollback")]
    pub rollback: bool,
}

/// <p>Optional deployment parameters that control how many tasks run during a deployment and the ordering of stopping and starting tasks.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeploymentConfiguration {
    /// <p><note> <p>The deployment circuit breaker can only be used for services using the rolling update (<code>ECS</code>) deployment type.</p> </note> <p>The <b>deployment circuit breaker</b> determines whether a service deployment will fail if the service can&#39;t reach a steady state. If deployment circuit breaker is enabled, a service deployment will transition to a failed state and stop launching new tasks. If rollback is enabled, when a service deployment fails, the service is rolled back to the last deployment that completed successfully.</p></p>
    #[serde(rename = "deploymentCircuitBreaker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_circuit_breaker: Option<DeploymentCircuitBreaker>,
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>maximumPercent</code> parameter represents an upper limit on the number of your service's tasks that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded down to the nearest integer). This parameter enables you to define the deployment batch size. For example, if your service is using the <code>REPLICA</code> service scheduler and has a <code>desiredCount</code> of four tasks and a <code>maximumPercent</code> value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default <code>maximumPercent</code> value for a service using the <code>REPLICA</code> service scheduler is 200%.</p> <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and tasks that use the EC2 launch type, the <b>maximum percent</b> value is set to the default value and is used to define the upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the maximum percent value is not used, although it is returned when describing your service.</p>
    #[serde(rename = "maximumPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_percent: Option<i64>,
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>minimumHealthyPercent</code> represents a lower limit on the number of your service's tasks that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded up to the nearest integer). This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>minimumHealthyPercent</code> of 50%, the service scheduler may stop two existing tasks to free up cluster capacity before starting two new tasks. </p> <p>For services that <i>do not</i> use a load balancer, the following should be noted:</p> <ul> <li> <p>A service is considered healthy if all essential containers within the tasks in the service pass their health checks.</p> </li> <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for 40 seconds after a task reaches a <code>RUNNING</code> state before the task is counted towards the minimum healthy percent total.</p> </li> <li> <p>If a task has one or more essential containers with a health check defined, the service scheduler will wait for the task to reach a healthy status before counting it towards the minimum healthy percent total. A task is considered healthy when all essential containers within the task have passed their health checks. The amount of time the service scheduler can wait for is determined by the container health check settings. </p> </li> </ul> <p>For services are that <i>do</i> use a load balancer, the following should be noted:</p> <ul> <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li> <li> <p>If a task has an essential container with a health check defined, the service scheduler will wait for both the task to reach a healthy status and the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li> </ul> <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the EC2 launch type, the <b>minimum healthy percent</b> value is set to the default value and is used to define the lower limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the Fargate launch type, the minimum healthy percent value is not used, although it is returned when describing your service.</p>
    #[serde(rename = "minimumHealthyPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_percent: Option<i64>,
}

/// <p>The deployment controller to use for the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeploymentController {
    /// <p><p>The deployment controller type to use.</p> <p>There are three deployment controller types available:</p> <dl> <dt>ECS</dt> <dd> <p>The rolling update (<code>ECS</code>) deployment type involves replacing the current running version of the container with the latest version. The number of containers Amazon ECS adds or removes from the service during a rolling update is controlled by adjusting the minimum and maximum number of healthy tasks allowed during a service deployment, as specified in the <a>DeploymentConfiguration</a>.</p> </dd> <dt>CODE<em>DEPLOY</dt> <dd> <p>The blue/green (<code>CODE</em>DEPLOY</code>) deployment type uses the blue/green deployment model powered by CodeDeploy, which allows you to verify a new deployment of a service before sending production traffic to it.</p> </dd> <dt>EXTERNAL</dt> <dd> <p>The external (<code>EXTERNAL</code>) deployment type enables you to use any third-party deployment controller for full control over the deployment process for an Amazon ECS service.</p> </dd> </dl></p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterContainerInstanceRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instance to deregister. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance ID or full ARN of the container instance to deregister. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    #[serde(rename = "containerInstance")]
    pub container_instance: String,
    /// <p>Forces the container instance to be deregistered. If you have tasks running on the container instance when you deregister it with the <code>force</code> option, these tasks remain running until you terminate the instance or the tasks stop through some other means, but they're orphaned (no longer monitored or accounted for by Amazon ECS). If an orphaned task on your container instance is part of an Amazon ECS service, then the service scheduler starts another copy of that task, on a different container instance if possible. </p> <p>Any containers in orphaned service tasks that are registered with a Classic Load Balancer or an Application Load Balancer target group are deregistered. They begin connection draining according to the settings on the load balancer or target group.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterContainerInstanceResponse {
    /// <p>The container instance that was deregistered.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterTaskDefinitionRequest {
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to deregister. You must specify a <code>revision</code>.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterTaskDefinitionResponse {
    /// <p>The full description of the deregistered task.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCapacityProvidersRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of one or more capacity providers. Up to <code>100</code> capacity providers can be described in an action.</p>
    #[serde(rename = "capacityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<String>>,
    /// <p>Specifies whether or not you want to see the resource tags for the capacity provider. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// <p>The maximum number of account setting results returned by <code>DescribeCapacityProviders</code> in paginated output. When this parameter is used, <code>DescribeCapacityProviders</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeCapacityProviders</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter is not used, then <code>DescribeCapacityProviders</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeCapacityProviders</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCapacityProvidersResponse {
    /// <p>The list of capacity providers.</p>
    #[serde(rename = "capacityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<CapacityProvider>>,
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeCapacityProviders</code> request. When the results of a <code>DescribeCapacityProviders</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeClustersRequest {
    /// <p>A list of up to 100 cluster names or full cluster Amazon Resource Name (ARN) entries. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    /// <p>Determines whether to include additional information about the clusters in the response. If this field is omitted, this information isn't included.</p> <p>If <code>ATTACHMENTS</code> is specified, the attachments for the container instances or tasks within the cluster are included.</p> <p>If <code>SETTINGS</code> is specified, the settings for the cluster are included.</p> <p>If <code>CONFIGURATIONS</code> is specified, the configuration for the cluster is included.</p> <p>If <code>STATISTICS</code> is specified, the task and service count is included, separated by launch type.</p> <p>If <code>TAGS</code> is specified, the metadata tags associated with the cluster are included.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClustersResponse {
    /// <p>The list of clusters.</p>
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeContainerInstancesRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to describe. If you do not specify a cluster, the default cluster is assumed. This parameter is required if the container instance or container instances you are describing were launched in any cluster other than the default cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>A list of up to 100 container instance IDs or full Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "containerInstances")]
    pub container_instances: Vec<String>,
    /// <p>Specifies whether you want to see the resource tags for the container instance. If <code>TAGS</code> is specified, the tags are included in the response. If <code>CONTAINER_INSTANCE_HEALTH</code> is specified, the container instance health is included in the response. If this field is omitted, tags and container instance health status aren't included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeContainerInstancesResponse {
    /// <p>The list of container instances.</p>
    #[serde(rename = "containerInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instances: Option<Vec<ContainerInstance>>,
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServicesRequest {
    /// <p>The short name or full Amazon Resource Name (ARN)the cluster that hosts the service to describe. If you do not specify a cluster, the default cluster is assumed. This parameter is required if the service or services you are describing were launched in any cluster other than the default cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Determines whether you want to see the resource tags for the service. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// <p>A list of services to describe. You may specify up to 10 services to describe in a single operation.</p>
    #[serde(rename = "services")]
    pub services: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeServicesResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>The list of services described.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTaskDefinitionRequest {
    /// <p>Determines whether to see the resource tags for the task definition. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// <p>The <code>family</code> for the latest <code>ACTIVE</code> revision, <code>family</code> and <code>revision</code> (<code>family:revision</code>) for a specific revision in the family, or full Amazon Resource Name (ARN) of the task definition to describe.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTaskDefinitionResponse {
    /// <p><p>The metadata that&#39;s applied to the task definition to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The full task definition description.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTaskSetsRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task sets exist in.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// <p>Specifies whether to see the resource tags for the task set. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the service that the task sets exist in.</p>
    #[serde(rename = "service")]
    pub service: String,
    /// <p>The ID or full Amazon Resource Name (ARN) of task sets to describe.</p>
    #[serde(rename = "taskSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTaskSetsResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>The list of task sets described.</p>
    #[serde(rename = "taskSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets: Option<Vec<TaskSet>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTasksRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task or tasks to describe. If you do not specify a cluster, the default cluster is assumed. This parameter is required if the task or tasks you are describing were launched in any cluster other than the default cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Specifies whether you want to see the resource tags for the task. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// <p>A list of up to 100 task IDs or full ARN entries.</p>
    #[serde(rename = "tasks")]
    pub tasks: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTasksResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>The list of tasks.</p>
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

/// <p>An object representing a container instance host device.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Device {
    /// <p>The path inside the container at which to expose the host device.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>The path for the device on the host container instance.</p>
    #[serde(rename = "hostPath")]
    pub host_path: String,
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DiscoverPollEndpointRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that the container instance belongs to.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance ID or full ARN of the container instance. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiscoverPollEndpointResponse {
    /// <p>The endpoint for the Amazon ECS agent to poll.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The telemetry endpoint for the Amazon ECS agent.</p>
    #[serde(rename = "telemetryEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_endpoint: Option<String>,
}

/// <p>This parameter is specified when you're using Docker volumes. Docker volumes are only supported when you're using the EC2 launch type. Windows containers only support the use of the <code>local</code> driver. To use bind mounts, specify a <code>host</code> instead.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DockerVolumeConfiguration {
    /// <p><p>If this value is <code>true</code>, the Docker volume is created if it doesn&#39;t already exist.</p> <note> <p>This field is only used if the <code>scope</code> is <code>shared</code>.</p> </note></p>
    #[serde(rename = "autoprovision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoprovision: Option<bool>,
    /// <p>The Docker volume driver to use. The driver value must match the driver name provided by Docker because it is used for task placement. If the driver was installed using the Docker plugin CLI, use <code>docker plugin ls</code> to retrieve the driver name from your container instance. If the driver was installed using another method, use Docker plugin discovery to retrieve the driver name. For more information, see <a href="https://docs.docker.com/engine/extend/plugin_api/#plugin-discovery">Docker plugin discovery</a>. This parameter maps to <code>Driver</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/VolumeCreate">Create a volume</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>xxdriver</code> option to <a href="https://docs.docker.com/engine/reference/commandline/volume_create/">docker volume create</a>.</p>
    #[serde(rename = "driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// <p>A map of Docker driver-specific options passed through. This parameter maps to <code>DriverOpts</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/VolumeCreate">Create a volume</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>xxopt</code> option to <a href="https://docs.docker.com/engine/reference/commandline/volume_create/">docker volume create</a>.</p>
    #[serde(rename = "driverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<::std::collections::HashMap<String, String>>,
    /// <p>Custom metadata to add to your Docker volume. This parameter maps to <code>Labels</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/VolumeCreate">Create a volume</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>xxlabel</code> option to <a href="https://docs.docker.com/engine/reference/commandline/volume_create/">docker volume create</a>.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>The scope for the Docker volume that determines its lifecycle. Docker volumes that are scoped to a <code>task</code> are automatically provisioned when the task starts and destroyed when the task stops. Docker volumes that are scoped as <code>shared</code> persist after the task stops.</p>
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// <p>The authorization configuration details for the Amazon EFS file system.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EFSAuthorizationConfig {
    /// <p>The Amazon EFS access point ID to use. If an access point is specified, the root directory value specified in the <code>EFSVolumeConfiguration</code> must either be omitted or set to <code>/</code> which will enforce the path set on the EFS access point. If an access point is used, transit encryption must be enabled in the <code>EFSVolumeConfiguration</code>. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-access-points.html">Working with Amazon EFS access points</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    #[serde(rename = "accessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    /// <p>Determines whether to use the Amazon ECS task IAM role defined in a task definition when mounting the Amazon EFS file system. If enabled, transit encryption must be enabled in the <code>EFSVolumeConfiguration</code>. If this parameter is omitted, the default value of <code>DISABLED</code> is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/efs-volumes.html#efs-volume-accesspoints">Using Amazon EFS access points</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "iam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<String>,
}

/// <p>This parameter is specified when you're using an Amazon Elastic File System file system for task storage. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/efs-volumes.html">Amazon EFS volumes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EFSVolumeConfiguration {
    /// <p>The authorization configuration details for the Amazon EFS file system.</p>
    #[serde(rename = "authorizationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_config: Option<EFSAuthorizationConfig>,
    /// <p>The Amazon EFS file system ID to use.</p>
    #[serde(rename = "fileSystemId")]
    pub file_system_id: String,
    /// <p><p>The directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume will be used. Specifying <code>/</code> will have the same effect as omitting this parameter.</p> <important> <p>If an EFS access point is specified in the <code>authorizationConfig</code>, the root directory parameter must either be omitted or set to <code>/</code> which will enforce the path set on the EFS access point.</p> </important></p>
    #[serde(rename = "rootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,
    /// <p>Determines whether to use encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server. Transit encryption must be enabled if Amazon EFS IAM authorization is used. If this parameter is omitted, the default value of <code>DISABLED</code> is used. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/encryption-in-transit.html">Encrypting data in transit</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    #[serde(rename = "transitEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<String>,
    /// <p>The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server. If you do not specify a transit encryption port, it will use the port selection strategy that the Amazon EFS mount helper uses. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-mount-helper.html">EFS mount helper</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    #[serde(rename = "transitEncryptionPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_port: Option<i64>,
}

/// <p><p>A list of files containing the environment variables to pass to a container. You can specify up to ten environment files. The file must have a <code>.env</code> file extension. Each line in an environment file should contain an environment variable in <code>VARIABLE=VALUE</code> format. Lines beginning with <code>#</code> are treated as comments and are ignored. For more information about the environment variable file syntax, see <a href="https://docs.docker.com/compose/env-file/">Declare default environment variables in file</a>.</p> <p>If there are environment variables specified using the <code>environment</code> parameter in a container definition, they take precedence over the variables contained within an environment file. If multiple environment files are specified that contain the same variable, they&#39;re processed from the top down. We recommend that you use unique variable names. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/taskdef-envfiles.html">Specifying environment variables</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>This parameter is only supported for tasks hosted on Fargate using the following platform versions:</p> <ul> <li> <p>Linux platform version <code>1.4.0</code> or later.</p> </li> <li> <p>Windows platform version <code>1.0.0</code> or later.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EnvironmentFile {
    /// <p>The file type to use. The only supported value is <code>s3</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon S3 object containing the environment variable file.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p><p>The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on Fargate. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/userguide/using_data_volumes.html">Fargate task storage</a> in the <i>Amazon ECS User Guide for Fargate</i>.</p> <note> <p>This parameter is only supported for tasks hosted on Fargate using Linux platform version <code>1.4.0</code> or later. This parameter is not supported for Windows containers on Fargate.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EphemeralStorage {
    /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
    #[serde(rename = "sizeInGiB")]
    pub size_in_gi_b: i64,
}

/// <p>The details of the execute command configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExecuteCommandConfiguration {
    /// <p>Specify an Key Management Service key ID to encrypt the data between the local client and the container.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The log configuration for the results of the execute command actions. The logs can be sent to CloudWatch Logs or an Amazon S3 bucket. When <code>logging=OVERRIDE</code> is specified, a <code>logConfiguration</code> must be provided.</p>
    #[serde(rename = "logConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<ExecuteCommandLogConfiguration>,
    /// <p><p>The log setting to use for redirecting logs for your execute command results. The following log settings are available.</p> <ul> <li> <p> <code>NONE</code>: The execute command session is not logged.</p> </li> <li> <p> <code>DEFAULT</code>: The <code>awslogs</code> configuration in the task definition is used. If no logging parameter is specified, it defaults to this value. If no <code>awslogs</code> log driver is configured in the task definition, the output won&#39;t be logged.</p> </li> <li> <p> <code>OVERRIDE</code>: Specify the logging details as a part of <code>logConfiguration</code>. If the <code>OVERRIDE</code> logging option is specified, the <code>logConfiguration</code> is required.</p> </li> </ul></p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,
}

/// <p>The log configuration for the results of the execute command actions. The logs can be sent to CloudWatch Logs or an Amazon S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExecuteCommandLogConfiguration {
    /// <p>Determines whether to use encryption on the CloudWatch logs. If not specified, encryption will be disabled.</p>
    #[serde(rename = "cloudWatchEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption_enabled: Option<bool>,
    /// <p><p>The name of the CloudWatch log group to send logs to.</p> <note> <p>The CloudWatch log group must already be created.</p> </note></p>
    #[serde(rename = "cloudWatchLogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_name: Option<String>,
    /// <p><p>The name of the S3 bucket to send logs to.</p> <note> <p>The S3 bucket must already be created.</p> </note></p>
    #[serde(rename = "s3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket_name: Option<String>,
    /// <p>Determines whether to use encryption on the S3 logs. If not specified, encryption is not used.</p>
    #[serde(rename = "s3EncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_encryption_enabled: Option<bool>,
    /// <p>An optional folder in the S3 bucket to place logs in.</p>
    #[serde(rename = "s3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key_prefix: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecuteCommandRequest {
    /// <p>The Amazon Resource Name (ARN) or short name of the cluster the task is running in. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The command to run on the container.</p>
    #[serde(rename = "command")]
    pub command: String,
    /// <p>The name of the container to execute the command on. A container name only needs to be specified for tasks containing multiple containers.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// <p>Use this flag to run your command in interactive mode.</p>
    #[serde(rename = "interactive")]
    pub interactive: bool,
    /// <p>The Amazon Resource Name (ARN) or ID of the task the container is part of.</p>
    #[serde(rename = "task")]
    pub task: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecuteCommandResponse {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the container.</p>
    #[serde(rename = "containerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arn: Option<String>,
    /// <p>The name of the container.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>Determines whether the execute command session is running in interactive mode. Amazon ECS only supports initiating interactive sessions, so you must specify <code>true</code> for this value.</p>
    #[serde(rename = "interactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    /// <p>The details of the SSM session that was created for this instance of execute-command.</p>
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
    /// <p>The Amazon Resource Name (ARN) of the task.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>The authorization configuration details for Amazon FSx for Windows File Server file system. See <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_FSxWindowsFileServerVolumeConfiguration.html">FSxWindowsFileServerVolumeConfiguration</a> in the <i>Amazon ECS API Reference</i>.</p> <p>For more information and the input format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/wfsx-volumes.html">Amazon FSx for Windows File Server Volumes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FSxWindowsFileServerAuthorizationConfig {
    /// <p>The authorization credential option to use. The authorization credential options can be provided using either the Amazon Resource Name (ARN) of an Secrets Manager secret or SSM Parameter Store parameter. The ARN refers to the stored credentials.</p>
    #[serde(rename = "credentialsParameter")]
    pub credentials_parameter: String,
    /// <p>A fully qualified domain name hosted by an <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_microsoft_ad.html">Directory Service</a> Managed Microsoft AD (Active Directory) or self-hosted AD on Amazon EC2.</p>
    #[serde(rename = "domain")]
    pub domain: String,
}

/// <p>This parameter is specified when you're using <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/what-is.html">Amazon FSx for Windows File Server</a> file system for task storage.</p> <p>For more information and the input format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/wfsx-volumes.html">Amazon FSx for Windows File Server volumes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FSxWindowsFileServerVolumeConfiguration {
    /// <p>The authorization configuration details for the Amazon FSx for Windows File Server file system.</p>
    #[serde(rename = "authorizationConfig")]
    pub authorization_config: FSxWindowsFileServerAuthorizationConfig,
    /// <p>The Amazon FSx for Windows File Server file system ID to use.</p>
    #[serde(rename = "fileSystemId")]
    pub file_system_id: String,
    /// <p>The directory within the Amazon FSx for Windows File Server file system to mount as the root directory inside the host.</p>
    #[serde(rename = "rootDirectory")]
    pub root_directory: String,
}

/// <p>A failed resource. For a list of common causes, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/api_failures_messages.html">API failure reasons</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Failure {
    /// <p>The Amazon Resource Name (ARN) of the failed resource.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The details of the failure.</p>
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// <p>The reason for the failure.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>The FireLens configuration for the container. This is used to specify and configure a log router for container logs. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_firelens.html">Custom log routing</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FirelensConfiguration {
    /// <p><p>The options to use when configuring the log router. This field is optional and can be used to specify a custom configuration file or to add additional metadata, such as the task, task definition, cluster, and container instance details to the log event. If specified, the syntax to use is <code>&quot;options&quot;:{&quot;enable-ecs-log-metadata&quot;:&quot;true|false&quot;,&quot;config-file-type:&quot;s3|file&quot;,&quot;config-file-value&quot;:&quot;arn:aws:s3:::mybucket/fluent.conf|filepath&quot;}</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_firelens.html#firelens-taskdef">Creating a task definition that uses a FireLens configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>Tasks hosted on Fargate only support the <code>file</code> configuration file type.</p> </note></p>
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The log router to use. The valid values are <code>fluentd</code> or <code>fluentbit</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p><p>An object representing a container health check. Health check parameters that are specified in a container definition override any Docker health checks that exist in the container image (such as those specified in a parent image or from the image&#39;s Dockerfile).</p> <note> <p>The Amazon ECS container agent only monitors and reports on the health checks specified in the task definition. Amazon ECS does not monitor Docker health checks that are embedded in a container image and not specified in the container definition. Health check parameters that are specified in a container definition override any Docker health checks that exist in the container image.</p> </note> <p>You can view the health status of both individual containers and a task with the DescribeTasks API operation or when viewing the task details in the console.</p> <p>The following describes the possible <code>healthStatus</code> values for a container:</p> <ul> <li> <p> <code>HEALTHY</code>-The container health check has passed successfully.</p> </li> <li> <p> <code>UNHEALTHY</code>-The container health check has failed.</p> </li> <li> <p> <code>UNKNOWN</code>-The container health check is being evaluated or there&#39;s no container health check defined.</p> </li> </ul> <p>The following describes the possible <code>healthStatus</code> values for a task. The container health check status of nonessential containers do not have an effect on the health status of a task.</p> <ul> <li> <p> <code>HEALTHY</code>-All essential containers within the task have passed their health checks.</p> </li> <li> <p> <code>UNHEALTHY</code>-One or more essential containers have failed their health check.</p> </li> <li> <p> <code>UNKNOWN</code>-The essential containers within the task are still having their health checks evaluated or there are no container health checks defined.</p> </li> </ul> <p>If a task is run manually, and not as part of a service, the task will continue its lifecycle regardless of its health status. For tasks that are part of a service, if the task reports as unhealthy then the task will be stopped and the service scheduler will replace it.</p> <p>The following are notes about container health check support:</p> <ul> <li> <p>Container health checks require version 1.17.0 or greater of the Amazon ECS container agent. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html">Updating the Amazon ECS container agent</a>.</p> </li> <li> <p>Container health checks are supported for Fargate tasks if you&#39;re using platform version <code>1.1.0</code> or greater. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate platform versions</a>.</p> </li> <li> <p>Container health checks aren&#39;t supported for tasks that are part of a service that&#39;s configured to use a Classic Load Balancer.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HealthCheck {
    /// <p>A string array representing the command that the container runs to determine if it is healthy. The string array must start with <code>CMD</code> to execute the command arguments directly, or <code>CMD-SHELL</code> to run the command with the container's default shell. </p> <p> When you use the Amazon Web Services Management Console JSON panel, the Command Line Interface, or the APIs, enclose the list of commands in brackets.</p> <p> <code>[ "CMD-SHELL", "curl -f http://localhost/ || exit 1" ]</code> </p> <p>You don't need to include the brackets when you use the Amazon Web Services Management Console.</p> <p> <code> "CMD-SHELL", "curl -f http://localhost/ || exit 1" </code> </p> <p>An exit code of 0 indicates success, and non-zero exit code indicates failure. For more information, see <code>HealthCheck</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a>.</p>
    #[serde(rename = "command")]
    pub command: Vec<String>,
    /// <p>The time period in seconds between each health check execution. You may specify between 5 and 300 seconds. The default value is 30 seconds.</p>
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The number of times to retry a failed health check before the container is considered unhealthy. You may specify between 1 and 10 retries. The default value is 3.</p>
    #[serde(rename = "retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    /// <p><p>The optional grace period to provide containers time to bootstrap before failed health checks count towards the maximum number of retries. You can specify between 0 and 300 seconds. By default, the <code>startPeriod</code> is disabled.</p> <note> <p>If a health check succeeds within the <code>startPeriod</code>, then the container is considered healthy and any subsequent failures count toward the maximum number of retries.</p> </note></p>
    #[serde(rename = "startPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i64>,
    /// <p>The time period in seconds to wait for a health check to succeed before it is considered a failure. You may specify between 2 and 60 seconds. The default value is 5.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// <p>Hostnames and IP address entries that are added to the <code>/etc/hosts</code> file of a container via the <code>extraHosts</code> parameter of its <a>ContainerDefinition</a>. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HostEntry {
    /// <p>The hostname to use in the <code>/etc/hosts</code> entry.</p>
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// <p>The IP address to use in the <code>/etc/hosts</code> entry.</p>
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}

/// <p>Details on a container instance bind mount host volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HostVolumeProperties {
    /// <p>When the <code>host</code> parameter is used, specify a <code>sourcePath</code> to declare the path on the host container instance that's presented to the container. If this parameter is empty, then the Docker daemon has assigned a host path for you. If the <code>host</code> parameter contains a <code>sourcePath</code> file location, then the data volume persists at the specified location on the host container instance until you delete it manually. If the <code>sourcePath</code> value doesn't exist on the host container instance, the Docker daemon creates it. If the location does exist, the contents of the source path folder are exported.</p> <p>If you're using the Fargate launch type, the <code>sourcePath</code> parameter is not supported.</p>
    #[serde(rename = "sourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>Details on an Elastic Inference accelerator. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-inference.html">Working with Amazon Elastic Inference on Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InferenceAccelerator {
    /// <p>The Elastic Inference accelerator device name. The <code>deviceName</code> must also be referenced in a container definition as a <a>ResourceRequirement</a>.</p>
    #[serde(rename = "deviceName")]
    pub device_name: String,
    /// <p>The Elastic Inference accelerator type to use.</p>
    #[serde(rename = "deviceType")]
    pub device_type: String,
}

/// <p>Details on an Elastic Inference accelerator task override. This parameter is used to override the Elastic Inference accelerator specified in the task definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-inference.html">Working with Amazon Elastic Inference on Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InferenceAcceleratorOverride {
    /// <p>The Elastic Inference accelerator device name to override for the task. This parameter must match a <code>deviceName</code> specified in the task definition.</p>
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The Elastic Inference accelerator type to use.</p>
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

/// <p>An object representing the result of a container instance health status check.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceHealthCheckResult {
    /// <p>The Unix timestamp for when the container instance health status last changed.</p>
    #[serde(rename = "lastStatusChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<f64>,
    /// <p>The Unix timestamp for when the container instance health status was last updated.</p>
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The container instance health status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of container instance health status that was verified.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker. For more information about the default capabilities and the non-default available capabilities, see <a href="https://docs.docker.com/engine/reference/run/#runtime-privilege-and-linux-capabilities">Runtime privilege and Linux capabilities</a> in the <i>Docker run reference</i>. For more detailed information about these Linux capabilities, see the <a href="http://man7.org/linux/man-pages/man7/capabilities.7.html">capabilities(7)</a> Linux manual page.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KernelCapabilities {
    /// <p>The Linux capabilities for the container that have been added to the default configuration provided by Docker. This parameter maps to <code>CapAdd</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--cap-add</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>Tasks launched on Fargate only support adding the <code>SYS_PTRACE</code> kernel capability.</p> </note> <p>Valid values: <code>"ALL" | "AUDIT_CONTROL" | "AUDIT_WRITE" | "BLOCK_SUSPEND" | "CHOWN" | "DAC_OVERRIDE" | "DAC_READ_SEARCH" | "FOWNER" | "FSETID" | "IPC_LOCK" | "IPC_OWNER" | "KILL" | "LEASE" | "LINUX_IMMUTABLE" | "MAC_ADMIN" | "MAC_OVERRIDE" | "MKNOD" | "NET_ADMIN" | "NET_BIND_SERVICE" | "NET_BROADCAST" | "NET_RAW" | "SETFCAP" | "SETGID" | "SETPCAP" | "SETUID" | "SYS_ADMIN" | "SYS_BOOT" | "SYS_CHROOT" | "SYS_MODULE" | "SYS_NICE" | "SYS_PACCT" | "SYS_PTRACE" | "SYS_RAWIO" | "SYS_RESOURCE" | "SYS_TIME" | "SYS_TTY_CONFIG" | "SYSLOG" | "WAKE_ALARM"</code> </p>
    #[serde(rename = "add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    /// <p>The Linux capabilities for the container that have been removed from the default configuration provided by Docker. This parameter maps to <code>CapDrop</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--cap-drop</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <p>Valid values: <code>"ALL" | "AUDIT_CONTROL" | "AUDIT_WRITE" | "BLOCK_SUSPEND" | "CHOWN" | "DAC_OVERRIDE" | "DAC_READ_SEARCH" | "FOWNER" | "FSETID" | "IPC_LOCK" | "IPC_OWNER" | "KILL" | "LEASE" | "LINUX_IMMUTABLE" | "MAC_ADMIN" | "MAC_OVERRIDE" | "MKNOD" | "NET_ADMIN" | "NET_BIND_SERVICE" | "NET_BROADCAST" | "NET_RAW" | "SETFCAP" | "SETGID" | "SETPCAP" | "SETUID" | "SYS_ADMIN" | "SYS_BOOT" | "SYS_CHROOT" | "SYS_MODULE" | "SYS_NICE" | "SYS_PACCT" | "SYS_PTRACE" | "SYS_RAWIO" | "SYS_RESOURCE" | "SYS_TIME" | "SYS_TTY_CONFIG" | "SYSLOG" | "WAKE_ALARM"</code> </p>
    #[serde(rename = "drop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

/// <p>A key-value pair object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KeyValuePair {
    /// <p>The name of the key-value pair. For environment variables, this is the name of the environment variable.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the key-value pair. For environment variables, this is the value of the environment variable.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Linux-specific options that are applied to the container, such as Linux <a>KernelCapabilities</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LinuxParameters {
    /// <p><p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker.</p> <note> <p>For tasks that use the Fargate launch type, <code>capabilities</code> is supported for all platform versions but the <code>add</code> parameter is only supported if using platform version 1.4.0 or later.</p> </note></p>
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<KernelCapabilities>,
    /// <p><p>Any host devices to expose to the container. This parameter maps to <code>Devices</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--device</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>If you&#39;re using tasks that use the Fargate launch type, the <code>devices</code> parameter isn&#39;t supported.</p> </note></p>
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    /// <p>Run an <code>init</code> process inside the container that forwards signals and reaps processes. This parameter maps to the <code>--init</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>. This parameter requires version 1.25 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format '{{.Server.APIVersion}}'</code> </p>
    #[serde(rename = "initProcessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,
    /// <p><p>The total amount of swap memory (in MiB) a container can use. This parameter will be translated to the <code>--memory-swap</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a> where the value would be the sum of the container memory plus the <code>maxSwap</code> value.</p> <p>If a <code>maxSwap</code> value of <code>0</code> is specified, the container will not use swap. Accepted values are <code>0</code> or any positive integer. If the <code>maxSwap</code> parameter is omitted, the container will use the swap configuration for the container instance it is running on. A <code>maxSwap</code> value must be set for the <code>swappiness</code> parameter to be used.</p> <note> <p>If you&#39;re using tasks that use the Fargate launch type, the <code>maxSwap</code> parameter isn&#39;t supported.</p> </note></p>
    #[serde(rename = "maxSwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_swap: Option<i64>,
    /// <p><p>The value for the size (in MiB) of the <code>/dev/shm</code> volume. This parameter maps to the <code>--shm-size</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>If you are using tasks that use the Fargate launch type, the <code>sharedMemorySize</code> parameter is not supported.</p> </note></p>
    #[serde(rename = "sharedMemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i64>,
    /// <p><p>This allows you to tune a container&#39;s memory swappiness behavior. A <code>swappiness</code> value of <code>0</code> will cause swapping to not happen unless absolutely necessary. A <code>swappiness</code> value of <code>100</code> will cause pages to be swapped very aggressively. Accepted values are whole numbers between <code>0</code> and <code>100</code>. If the <code>swappiness</code> parameter is not specified, a default value of <code>60</code> is used. If a value is not specified for <code>maxSwap</code> then this parameter is ignored. This parameter maps to the <code>--memory-swappiness</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>If you&#39;re using tasks that use the Fargate launch type, the <code>swappiness</code> parameter isn&#39;t supported.</p> </note></p>
    #[serde(rename = "swappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i64>,
    /// <p><p>The container path, mount options, and size (in MiB) of the tmpfs mount. This parameter maps to the <code>--tmpfs</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <note> <p>If you&#39;re using tasks that use the Fargate launch type, the <code>tmpfs</code> parameter isn&#39;t supported.</p> </note></p>
    #[serde(rename = "tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<Tmpfs>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAccountSettingsRequest {
    /// <p>Determines whether to return the effective settings. If <code>true</code>, the account settings for the root user or the default setting for the <code>principalArn</code> are returned. If <code>false</code>, the account settings for the <code>principalArn</code> are returned if they're set. Otherwise, no account settings are returned.</p>
    #[serde(rename = "effectiveSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_settings: Option<bool>,
    /// <p>The maximum number of account setting results returned by <code>ListAccountSettings</code> in paginated output. When this parameter is used, <code>ListAccountSettings</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListAccountSettings</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter isn't used, then <code>ListAccountSettings</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the account setting you want to list the settings for.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The <code>nextToken</code> value returned from a <code>ListAccountSettings</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it&#39;s possible the number of results to be fewer than <code>maxResults</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. If this field is omitted, the account settings are listed only for the authenticated user.</p> <note> <p>Federated users assume the account setting of the root user and can&#39;t have explicit account settings set for them.</p> </note></p>
    #[serde(rename = "principalArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    /// <p>The value of the account settings to filter results with. You must also specify an account setting name to use this parameter.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAccountSettingsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListAccountSettings</code> request. When the results of a <code>ListAccountSettings</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The account settings for the resource.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<Setting>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAttributesRequest {
    /// <p>The name of the attribute to filter the results with. </p>
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The value of the attribute to filter results with. You must also specify an attribute name to use this parameter.</p>
    #[serde(rename = "attributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to list attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The maximum number of cluster results that <code>ListAttributes</code> returned in paginated output. When this parameter is used, <code>ListAttributes</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListAttributes</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListAttributes</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a <code>ListAttributes</code> request indicating that more results are available to fulfill the request and further calls are needed. If <code>maxResults</code> was provided, it&#39;s possible the number of results to be fewer than <code>maxResults</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of the target to list attributes with.</p>
    #[serde(rename = "targetType")]
    pub target_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAttributesResponse {
    /// <p>A list of attribute objects that meet the criteria of the request.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListAttributes</code> request. When the results of a <code>ListAttributes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListClustersRequest {
    /// <p>The maximum number of cluster results that <code>ListClusters</code> returned in paginated output. When this parameter is used, <code>ListClusters</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListClusters</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListClusters</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a <code>ListClusters</code> request indicating that more results are available to fulfill the request and further calls are needed. If <code>maxResults</code> was provided, it&#39;s possible the number of results to be fewer than <code>maxResults</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListClustersResponse {
    /// <p>The list of full Amazon Resource Name (ARN) entries for each cluster that's associated with your account.</p>
    #[serde(rename = "clusterArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arns: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListClusters</code> request. When the results of a <code>ListClusters</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListContainerInstancesRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to list. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// <p>The maximum number of container instance results that <code>ListContainerInstances</code> returned in paginated output. When this parameter is used, <code>ListContainerInstances</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListContainerInstances</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListContainerInstances</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a <code>ListContainerInstances</code> request indicating that more results are available to fulfill the request and further calls are needed. If <code>maxResults</code> was provided, it&#39;s possible the number of results to be fewer than <code>maxResults</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filters the container instances by status. For example, if you specify the <code>DRAINING</code> status, the results include only container instances that have been set to <code>DRAINING</code> using <a>UpdateContainerInstancesState</a>. If you don't specify this parameter, the default is to include container instances set to all states other than <code>INACTIVE</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListContainerInstancesResponse {
    /// <p>The list of container instances with full ARN entries for each container instance associated with the specified cluster.</p>
    #[serde(rename = "containerInstanceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arns: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListContainerInstances</code> request. When the results of a <code>ListContainerInstances</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServicesRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to use when filtering the <code>ListServices</code> results. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The launch type to use when filtering the <code>ListServices</code> results.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>The maximum number of service results that <code>ListServices</code> returned in paginated output. When this parameter is used, <code>ListServices</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListServices</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListServices</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a <code>ListServices</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The scheduling strategy to use when filtering the <code>ListServices</code> results.</p>
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServicesResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListServices</code> request. When the results of a <code>ListServices</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of full ARN entries for each service that's associated with the specified cluster.</p>
    #[serde(rename = "serviceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource to list the tags for. Currently, the supported resources are Amazon ECS tasks, services, task definitions, clusters, and container instances.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTaskDefinitionFamiliesRequest {
    /// <p>The <code>familyPrefix</code> is a string that's used to filter the results of <code>ListTaskDefinitionFamilies</code>. If you specify a <code>familyPrefix</code>, only task definition family names that begin with the <code>familyPrefix</code> string are returned.</p>
    #[serde(rename = "familyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_prefix: Option<String>,
    /// <p>The maximum number of task definition family results that <code>ListTaskDefinitionFamilies</code> returned in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitionFamilies</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListTaskDefinitionFamilies</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a <code>ListTaskDefinitionFamilies</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The task definition family status to filter the <code>ListTaskDefinitionFamilies</code> results with. By default, both <code>ACTIVE</code> and <code>INACTIVE</code> task definition families are listed. If this parameter is set to <code>ACTIVE</code>, only task definition families that have an <code>ACTIVE</code> task definition revision are returned. If this parameter is set to <code>INACTIVE</code>, only task definition families that do not have any <code>ACTIVE</code> task definition revisions are returned. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTaskDefinitionFamiliesResponse {
    /// <p>The list of task definition family names that match the <code>ListTaskDefinitionFamilies</code> request.</p>
    #[serde(rename = "families")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub families: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListTaskDefinitionFamilies</code> request. When the results of a <code>ListTaskDefinitionFamilies</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTaskDefinitionsRequest {
    /// <p>The full family name to filter the <code>ListTaskDefinitions</code> results with. Specifying a <code>familyPrefix</code> limits the listed task definitions to task definition revisions that belong to that family.</p>
    #[serde(rename = "familyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_prefix: Option<String>,
    /// <p>The maximum number of task definition results that <code>ListTaskDefinitions</code> returned in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitions</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListTaskDefinitions</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a <code>ListTaskDefinitions</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The order to sort the results in. Valid values are <code>ASC</code> and <code>DESC</code>. By default, (<code>ASC</code>) task definitions are listed lexicographically by family name and in ascending numerical order by revision so that the newest task definitions in a family are listed last. Setting this parameter to <code>DESC</code> reverses the sort order on family name and revision. This is so that the newest task definitions in a family are listed first.</p>
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// <p>The task definition status to filter the <code>ListTaskDefinitions</code> results with. By default, only <code>ACTIVE</code> task definitions are listed. By setting this parameter to <code>INACTIVE</code>, you can view task definitions that are <code>INACTIVE</code> as long as an active task or service still references them. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTaskDefinitionsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListTaskDefinitions</code> request. When the results of a <code>ListTaskDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of task definition Amazon Resource Name (ARN) entries for the <code>ListTaskDefinitions</code> request.</p>
    #[serde(rename = "taskDefinitionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTasksRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to use when filtering the <code>ListTasks</code> results. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance ID or full ARN of the container instance to use when filtering the <code>ListTasks</code> results. Specifying a <code>containerInstance</code> limits the results to tasks that belong to that container instance.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<String>,
    /// <p><p>The task desired status to use when filtering the <code>ListTasks</code> results. Specifying a <code>desiredStatus</code> of <code>STOPPED</code> limits the results to tasks that Amazon ECS has set the desired status to <code>STOPPED</code>. This can be useful for debugging tasks that aren&#39;t starting properly or have died or finished. The default status filter is <code>RUNNING</code>, which shows tasks that Amazon ECS has set the desired status to <code>RUNNING</code>.</p> <note> <p>Although you can filter results based on a desired status of <code>PENDING</code>, this doesn&#39;t return any results. Amazon ECS never sets the desired status of a task to that value (only a task&#39;s <code>lastStatus</code> may have a value of <code>PENDING</code>).</p> </note></p>
    #[serde(rename = "desiredStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<String>,
    /// <p>The name of the task definition family to use when filtering the <code>ListTasks</code> results. Specifying a <code>family</code> limits the results to tasks that belong to that family.</p>
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The launch type to use when filtering the <code>ListTasks</code> results.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>The maximum number of task results that <code>ListTasks</code> returned in paginated output. When this parameter is used, <code>ListTasks</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTasks</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListTasks</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a <code>ListTasks</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it&#39;s possible the number of results to be fewer than <code>maxResults</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the service to use when filtering the <code>ListTasks</code> results. Specifying a <code>serviceName</code> limits the results to tasks that belong to that service.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The <code>startedBy</code> value to filter the task results with. Specifying a <code>startedBy</code> value limits the results to tasks that were started with that value.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTasksResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListTasks</code> request. When the results of a <code>ListTasks</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of task ARN entries for the <code>ListTasks</code> request.</p>
    #[serde(rename = "taskArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arns: Option<Vec<String>>,
}

/// <p>The load balancer configuration to use with a service or task set.</p> <p>For specific notes and restrictions regarding the use of load balancers with services and task sets, see the CreateService and CreateTaskSet actions.</p> <p>When you add, update, or remove a load balancer configuration, Amazon ECS starts a new deployment with the updated Elastic Load Balancing configuration. This causes tasks to register to and deregister from load balancers.</p> <p>We recommend that you verify this on a test environment before you update the Elastic Load Balancing configuration. </p> <p>A service-linked role is required for services that use multiple target groups. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using service-linked roles</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LoadBalancer {
    /// <p>The name of the container (as it appears in a container definition) to associate with the load balancer.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The port on the container to associate with the load balancer. This port must correspond to a <code>containerPort</code> in the task definition the tasks in the service are using. For tasks that use the EC2 launch type, the container instance they're launched on must allow ingress traffic on the <code>hostPort</code> of the port mapping.</p>
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The name of the load balancer to associate with the Amazon ECS service or task set.</p> <p>A load balancer name is only specified when using a Classic Load Balancer. If you are using an Application Load Balancer or a Network Load Balancer the load balancer name parameter should be omitted.</p>
    #[serde(rename = "loadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// <p><p>The full Amazon Resource Name (ARN) of the Elastic Load Balancing target group or groups associated with a service or task set.</p> <p>A target group ARN is only specified when using an Application Load Balancer or Network Load Balancer. If you&#39;re using a Classic Load Balancer, omit the target group ARN.</p> <p>For services using the <code>ECS</code> deployment controller, you can specify one or multiple target groups. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/register-multiple-targetgroups.html">Registering multiple target groups with a service</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>For services using the <code>CODE_DEPLOY</code> deployment controller, you&#39;re required to define two target groups for the load balancer. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-type-bluegreen.html">Blue/green deployment with CodeDeploy</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <important> <p>If your service&#39;s task definition uses the <code>awsvpc</code> network mode, you must choose <code>ip</code> as the target type, not <code>instance</code>. Do this when creating your target groups because tasks that use the <code>awsvpc</code> network mode are associated with an elastic network interface, not an Amazon EC2 instance. This network mode is required for the Fargate launch type.</p> </important></p>
    #[serde(rename = "targetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
}

/// <p><p>The log configuration for the container. This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/commandline/run/"> <code>docker run</code> </a>.</p> <p>By default, containers use the same logging driver that the Docker daemon uses. However, the container might use a different logging driver than the Docker daemon by specifying a log driver configuration in the container definition. For more information about the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p> <p>Understand the following when specifying a log configuration for your containers.</p> <ul> <li> <p>Amazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the valid values below). Additional log drivers may be available in future releases of the Amazon ECS container agent.</p> </li> <li> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance.</p> </li> <li> <p>For tasks that are hosted on Amazon EC2 instances, the Amazon ECS container agent must register the available logging drivers with the <code>ECS<em>AVAILABLE</em>LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS container agent configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li> <li> <p>For tasks that are on Fargate, because you don&#39;t have access to the underlying infrastructure your tasks are hosted on, any additional software needed must be installed outside of the task. For example, the Fluentd output aggregators or a remote host running Logstash to send Gelf logs to.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LogConfiguration {
    /// <p><p>The log driver to use for the container.</p> <p>For tasks on Fargate, the supported log drivers are <code>awslogs</code>, <code>splunk</code>, and <code>awsfirelens</code>.</p> <p>For tasks hosted on Amazon EC2 instances, the supported log drivers are <code>awslogs</code>, <code>fluentd</code>, <code>gelf</code>, <code>json-file</code>, <code>journald</code>, <code>logentries</code>,<code>syslog</code>, <code>splunk</code>, and <code>awsfirelens</code>.</p> <p>For more information about using the <code>awslogs</code> log driver, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_awslogs.html">Using the awslogs log driver</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>For more information about using the <code>awsfirelens</code> log driver, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_firelens.html">Custom log routing</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>If you have a custom driver that isn&#39;t listed, you can fork the Amazon ECS container agent project that&#39;s <a href="https://github.com/aws/amazon-ecs-agent">available on GitHub</a> and customize it to work with that driver. We encourage you to submit pull requests for changes that you would like to have included. However, we don&#39;t currently provide support for running modified copies of this software.</p> </note></p>
    #[serde(rename = "logDriver")]
    pub log_driver: String,
    /// <p>The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format '{{.Server.APIVersion}}'</code> </p>
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The secrets to pass to the log configuration. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/specifying-sensitive-data.html">Specifying sensitive data</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "secretOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options: Option<Vec<Secret>>,
}

/// <p>Details about the managed agent status for the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ManagedAgent {
    /// <p>The Unix timestamp for the time when the managed agent was last started.</p>
    #[serde(rename = "lastStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_started_at: Option<f64>,
    /// <p>The last known status of the managed agent.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The name of the managed agent. When the execute command feature is enabled, the managed agent name is <code>ExecuteCommandAgent</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The reason for why the managed agent is in the state it is in.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>An object representing a change in state for a managed agent.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ManagedAgentStateChange {
    /// <p>The name of the container that's associated with the managed agent.</p>
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// <p>The name of the managed agent.</p>
    #[serde(rename = "managedAgentName")]
    pub managed_agent_name: String,
    /// <p>The reason for the status of the managed agent.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The status of the managed agent.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>The managed scaling settings for the Auto Scaling group capacity provider.</p> <p>When managed scaling is enabled, Amazon ECS manages the scale-in and scale-out actions of the Auto Scaling group. Amazon ECS manages a target tracking scaling policy using an Amazon ECS managed CloudWatch metric with the specified <code>targetCapacity</code> value as the target value for the metric. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/asg-capacity-providers.html#asg-capacity-providers-managed-scaling">Using managed scaling</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>If managed scaling is disabled, the user must manage the scaling of the Auto Scaling group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ManagedScaling {
    /// <p>The period of time, in seconds, after a newly launched Amazon EC2 instance can contribute to CloudWatch metrics for Auto Scaling group. If this parameter is omitted, the default value of <code>300</code> seconds is used.</p>
    #[serde(rename = "instanceWarmupPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_warmup_period: Option<i64>,
    /// <p>The maximum number of container instances that Amazon ECS scales in or scales out at one time. If this parameter is omitted, the default value of <code>10000</code> is used.</p>
    #[serde(rename = "maximumScalingStepSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_scaling_step_size: Option<i64>,
    /// <p>The minimum number of container instances that Amazon ECS scales in or scales out at one time. If this parameter is omitted, the default value of <code>1</code> is used.</p>
    #[serde(rename = "minimumScalingStepSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_scaling_step_size: Option<i64>,
    /// <p>Determines whether to use managed scaling for the capacity provider.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The target capacity value for the capacity provider. The specified value must be greater than <code>0</code> and less than or equal to <code>100</code>. A value of <code>100</code> results in the Amazon EC2 instances in your Auto Scaling group being completely used.</p>
    #[serde(rename = "targetCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_capacity: Option<i64>,
}

/// <p>Details for a volume mount point that's used in a container definition.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MountPoint {
    /// <p>The path on the container to mount the host volume at.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. If this value is <code>false</code>, then the container can write to the volume. The default value is <code>false</code>.</p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of the volume to mount. Must be a volume name referenced in the <code>name</code> parameter of task definition <code>volume</code>.</p>
    #[serde(rename = "sourceVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

/// <p>Details on the network bindings between a container and its host container instance. After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <code>networkBindings</code> section of <a>DescribeTasks</a> API responses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkBinding {
    /// <p>The IP address that the container is bound to on the container instance.</p>
    #[serde(rename = "bindIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_ip: Option<String>,
    /// <p>The port number on the container that's used with the network binding.</p>
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The port number on the host that's used with the network binding.</p>
    #[serde(rename = "hostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i64>,
    /// <p>The protocol used for the network binding.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// <p>An object representing the network configuration for a task or service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkConfiguration {
    /// <p><p>The VPC subnets and security groups that are associated with a task.</p> <note> <p>All specified subnets and security groups must be from the same VPC.</p> </note></p>
    #[serde(rename = "awsvpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

/// <p>An object representing the elastic network interface for tasks that use the <code>awsvpc</code> network mode.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkInterface {
    /// <p>The attachment ID for the network interface.</p>
    #[serde(rename = "attachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>The private IPv6 address for the network interface.</p>
    #[serde(rename = "ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_address: Option<String>,
    /// <p>The private IPv4 address for the network interface.</p>
    #[serde(rename = "privateIpv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ipv_4_address: Option<String>,
}

/// <p><p>An object representing a constraint on task placement. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html">Task placement constraints</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>If you&#39;re using the Fargate launch type, task placement constraints aren&#39;t supported.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlacementConstraint {
    /// <p>A cluster query language expression to apply to the constraint. The expression can have a maximum length of 2000 characters. You can't specify an expression if the constraint type is <code>distinctInstance</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster query language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The type of constraint. Use <code>distinctInstance</code> to ensure that each task in a particular group is running on a different container instance. Use <code>memberOf</code> to restrict the selection to a group of valid candidates.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The task placement strategy for a task or service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-strategies.html">Task placement strategies</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlacementStrategy {
    /// <p>The field to apply the placement strategy against. For the <code>spread</code> placement strategy, valid values are <code>instanceId</code> (or <code>host</code>, which has the same effect), or any platform or custom attribute that's applied to a container instance, such as <code>attribute:ecs.availability-zone</code>. For the <code>binpack</code> placement strategy, valid values are <code>cpu</code> and <code>memory</code>. For the <code>random</code> placement strategy, this field is not used.</p>
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// <p>The type of placement strategy. The <code>random</code> placement strategy randomly places tasks on available candidates. The <code>spread</code> placement strategy spreads placement across available candidates evenly based on the <code>field</code> parameter. The <code>binpack</code> strategy places tasks on available candidates that have the least available amount of the resource that's specified with the <code>field</code> parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory but still enough to run the task.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The devices that are available on the container instance. The only supported device type is a GPU.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PlatformDevice {
    /// <p>The ID for the GPUs on the container instance. The available GPU IDs can also be obtained on the container instance in the <code>/var/lib/ecs/gpu/nvidia_gpu_info.json</code> file.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The type of device that's available on the container instance. The only supported value is <code>GPU</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Port mappings allow containers to access ports on the host container instance to send or receive traffic. Port mappings are specified as part of the container definition.</p> <p>If you use containers in a task with the <code>awsvpc</code> or <code>host</code> network mode, specify the exposed ports using <code>containerPort</code>. The <code>hostPort</code> can be left blank or it must be the same value as the <code>containerPort</code>.</p> <note> <p>You can't expose the same container port for multiple protocols. If you attempt this, an error is returned.</p> </note> <p>After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <code>networkBindings</code> section of <a>DescribeTasks</a> API responses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortMapping {
    /// <p>The port number on the container that's bound to the user-specified or automatically assigned host port.</p> <p>If you use containers in a task with the <code>awsvpc</code> or <code>host</code> network mode, specify the exposed ports using <code>containerPort</code>.</p> <p>If you use containers in a task with the <code>bridge</code> network mode and you specify a container port and not a host port, your container automatically receives a host port in the ephemeral port range. For more information, see <code>hostPort</code>. Port mappings that are automatically assigned in this way do not count toward the 100 reserved ports limit of a container instance.</p>
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The port number on the container instance to reserve for your container.</p> <p>If you use containers in a task with the <code>awsvpc</code> or <code>host</code> network mode, the <code>hostPort</code> can either be left blank or set to the same value as the <code>containerPort</code>.</p> <p>If you use containers in a task with the <code>bridge</code> network mode, you can specify a non-reserved host port for your container port mapping, or you can omit the <code>hostPort</code> (or set it to <code>0</code>) while specifying a <code>containerPort</code> and your container automatically receives a port in the ephemeral port range for your container instance operating system and Docker version.</p> <p>The default ephemeral port range for Docker version 1.6.0 and later is listed on the instance under <code>/proc/sys/net/ipv4/ip_local_port_range</code>. If this kernel parameter is unavailable, the default ephemeral port range from 49153 through 65535 is used. Do not attempt to specify a host port in the ephemeral port range as these are reserved for automatic assignment. In general, ports below 32768 are outside of the ephemeral port range.</p> <note> <p>The default ephemeral port range from 49153 through 65535 is always used for Docker versions before 1.6.0.</p> </note> <p>The default reserved ports are 22 for SSH, the Docker ports 2375 and 2376, and the Amazon ECS container agent ports 51678-51680. Any host port that was previously specified in a running task is also reserved while the task is running. That is, after a task stops, the host port is released. The current reserved ports are displayed in the <code>remainingResources</code> of <a>DescribeContainerInstances</a> output. A container instance can have up to 100 reserved ports at a time. This number includes the default reserved ports. Automatically assigned ports aren't included in the 100 reserved ports quota.</p>
    #[serde(rename = "hostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i64>,
    /// <p>The protocol used for the port mapping. Valid values are <code>tcp</code> and <code>udp</code>. The default is <code>tcp</code>.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// <p>The configuration details for the App Mesh proxy.</p> <p>For tasks that use the EC2 launch type, the container instances require at least version 1.26.0 of the container agent and at least version 1.26.0-1 of the <code>ecs-init</code> package to use a proxy configuration. If your container instances are launched from the Amazon ECS optimized AMI version <code>20190301</code> or later, then they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized Linux AMI</a> </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProxyConfiguration {
    /// <p>The name of the container that will serve as the App Mesh proxy.</p>
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// <p><p>The set of network configuration parameters to provide the Container Network Interface (CNI) plugin, specified as key-value pairs.</p> <ul> <li> <p> <code>IgnoredUID</code> - (Required) The user ID (UID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredGID</code> is specified, this field can be empty.</p> </li> <li> <p> <code>IgnoredGID</code> - (Required) The group ID (GID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredUID</code> is specified, this field can be empty.</p> </li> <li> <p> <code>AppPorts</code> - (Required) The list of ports that the application uses. Network traffic to these ports is forwarded to the <code>ProxyIngressPort</code> and <code>ProxyEgressPort</code>.</p> </li> <li> <p> <code>ProxyIngressPort</code> - (Required) Specifies the port that incoming traffic to the <code>AppPorts</code> is directed to.</p> </li> <li> <p> <code>ProxyEgressPort</code> - (Required) Specifies the port that outgoing traffic from the <code>AppPorts</code> is directed to.</p> </li> <li> <p> <code>EgressIgnoredPorts</code> - (Required) The egress traffic going to the specified ports is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li> <li> <p> <code>EgressIgnoredIPs</code> - (Required) The egress traffic going to the specified IP addresses is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li> </ul></p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<KeyValuePair>>,
    /// <p>The proxy type. The only supported value is <code>APPMESH</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccountSettingDefaultRequest {
    /// <p>The resource name for which to modify the account setting. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the ENI limit for your Amazon ECS container instances is affected. If <code>containerInsights</code> is specified, the default setting for CloudWatch Container Insights for your clusters is affected.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccountSettingDefaultResponse {
    /// <p>The current setting for a resource.</p>
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccountSettingRequest {
    /// <p>The Amazon ECS resource name for which to modify the account setting. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for your Amazon ECS container instances is affected. If <code>containerInsights</code> is specified, the default setting for CloudWatch Container Insights for your clusters is affected.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. If you specify the root user, it modifies the account setting for all IAM users, IAM roles, and the root user of the account unless an IAM user or role explicitly overrides these settings. If this field is omitted, the setting is changed only for the authenticated user.</p> <note> <p>Federated users assume the account setting of the root user and can&#39;t have explicit account settings set for them.</p> </note></p>
    #[serde(rename = "principalArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccountSettingResponse {
    /// <p>The current account setting for a resource.</p>
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAttributesRequest {
    /// <p>The attributes to apply to your resource. You can specify up to 10 custom attributes for each resource. You can specify up to 10 attributes in a single call.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to apply attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAttributesResponse {
    /// <p>The attributes applied to your resource.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutClusterCapacityProvidersRequest {
    /// <p>The name of one or more capacity providers to associate with the cluster.</p> <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must already be created. New capacity providers can be created with the <a>CreateCapacityProvider</a> API operation.</p> <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p>
    #[serde(rename = "capacityProviders")]
    pub capacity_providers: Vec<String>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to modify the capacity provider settings for. If you don't specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// <p>The capacity provider strategy to use by default for the cluster.</p> <p>When creating a service or running a task on a cluster, if no capacity provider or launch type is specified then the default capacity provider strategy for the cluster is used.</p> <p>A capacity provider strategy consists of one or more capacity providers along with the <code>base</code> and <code>weight</code> to assign to them. A capacity provider must be associated with the cluster to be used in a capacity provider strategy. The <a>PutClusterCapacityProviders</a> API is used to associate a capacity provider with a cluster. Only capacity providers with an <code>ACTIVE</code> or <code>UPDATING</code> status can be used.</p> <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must already be created. New capacity providers can be created with the <a>CreateCapacityProvider</a> API operation.</p> <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p>
    #[serde(rename = "defaultCapacityProviderStrategy")]
    pub default_capacity_provider_strategy: Vec<CapacityProviderStrategyItem>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutClusterCapacityProvidersResponse {
    /// <p>Details about the cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterContainerInstanceRequest {
    /// <p>The container instance attributes that this container instance supports.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to register your container instance with. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The ARN of the container instance (if it was previously registered).</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The instance identity document for the EC2 instance to register. This document can be found by running the following command from the instance: <code>curl http://169.254.169.254/latest/dynamic/instance-identity/document/</code> </p>
    #[serde(rename = "instanceIdentityDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity_document: Option<String>,
    /// <p>The instance identity document signature for the EC2 instance to register. This signature can be found by running the following command from the instance: <code>curl http://169.254.169.254/latest/dynamic/instance-identity/signature/</code> </p>
    #[serde(rename = "instanceIdentityDocumentSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity_document_signature: Option<String>,
    /// <p>The devices that are available on the container instance. The only supported device type is a GPU.</p>
    #[serde(rename = "platformDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_devices: Option<Vec<PlatformDevice>>,
    /// <p><p>The metadata that you apply to the container instance to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The resources available on the instance.</p>
    #[serde(rename = "totalResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resources: Option<Vec<Resource>>,
    /// <p>The version information for the Amazon ECS container agent and Docker daemon that runs on the container instance.</p>
    #[serde(rename = "versionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<VersionInfo>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterContainerInstanceResponse {
    /// <p>The container instance that was registered.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterTaskDefinitionRequest {
    /// <p>A list of container definitions in JSON format that describe the different containers that make up your task.</p>
    #[serde(rename = "containerDefinitions")]
    pub container_definitions: Vec<ContainerDefinition>,
    /// <p><p>The number of CPU units used by the task. It can be expressed as an integer using CPU units (for example, <code>1024</code>) or as a string using vCPUs (for example, <code>1 vCPU</code> or <code>1 vcpu</code>) in a task definition. String values are converted to an integer indicating the CPU units when the task definition is registered.</p> <note> <p>Task-level CPU and memory parameters are ignored for Windows containers. We recommend specifying container-level resources for Windows containers.</p> </note> <p>If you&#39;re using the EC2 launch type, this field is optional. Supported values are between <code>128</code> CPU units (<code>0.125</code> vCPUs) and <code>10240</code> CPU units (<code>10</code> vCPUs).</p> <p>If you&#39;re using the Fargate launch type, this field is required and you must use one of the following values, which determines your range of supported values for the <code>memory</code> parameter:</p> <p>The CPU units cannot be less than 1 vCPU when you use Windows containers on Fargate.</p> <ul> <li> <p>256 (.25 vCPU) - Available <code>memory</code> values: 512 (0.5 GB), 1024 (1 GB), 2048 (2 GB)</p> </li> <li> <p>512 (.5 vCPU) - Available <code>memory</code> values: 1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB)</p> </li> <li> <p>1024 (1 vCPU) - Available <code>memory</code> values: 2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB)</p> </li> <li> <p>2048 (2 vCPU) - Available <code>memory</code> values: Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB)</p> </li> <li> <p>4096 (4 vCPU) - Available <code>memory</code> values: Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB)</p> </li> </ul></p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p><p>The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on Fargate. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/userguide/using_data_volumes.html">Fargate task storage</a> in the <i>Amazon ECS User Guide for Fargate</i>.</p> <note> <p>This parameter is only supported for tasks hosted on Fargate using the following platform versions:</p> <ul> <li> <p>Linux platform version <code>1.4.0</code> or later.</p> </li> <li> <p>Windows platform version <code>1.0.0</code> or later.</p> </li> </ul> </note></p>
    #[serde(rename = "ephemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    /// <p>The Amazon Resource Name (ARN) of the task execution role that grants the Amazon ECS container agent permission to make Amazon Web Services API calls on your behalf. The task execution IAM role is required depending on the requirements of your task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_execution_IAM_role.html">Amazon ECS task execution IAM role</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>You must specify a <code>family</code> for a task definition. You can use it track multiple versions of the same task definition. The <code>family</code> is used as a name for your task definition. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed.</p>
    #[serde(rename = "family")]
    pub family: String,
    /// <p>The Elastic Inference accelerators to use for the containers in the task.</p>
    #[serde(rename = "inferenceAccelerators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<InferenceAccelerator>>,
    /// <p><p>The IPC resource namespace to use for the containers in the task. The valid values are <code>host</code>, <code>task</code>, or <code>none</code>. If <code>host</code> is specified, then all containers within the tasks that specified the <code>host</code> IPC mode on the same container instance share the same IPC resources with the host Amazon EC2 instance. If <code>task</code> is specified, all containers within the specified task share the same IPC resources. If <code>none</code> is specified, then IPC resources within the containers of a task are private and not shared with other containers in a task or on the container instance. If no value is specified, then the IPC resource namespace sharing depends on the Docker daemon setting on the container instance. For more information, see <a href="https://docs.docker.com/engine/reference/run/#ipc-settings---ipc">IPC settings</a> in the <i>Docker run reference</i>.</p> <p>If the <code>host</code> IPC mode is used, be aware that there is a heightened risk of undesired IPC namespace expose. For more information, see <a href="https://docs.docker.com/engine/security/security/">Docker security</a>.</p> <p>If you are setting namespaced kernel parameters using <code>systemControls</code> for the containers in the task, the following will apply to your IPC resource namespace. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html">System Controls</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <ul> <li> <p>For tasks that use the <code>host</code> IPC mode, IPC namespace related <code>systemControls</code> are not supported.</p> </li> <li> <p>For tasks that use the <code>task</code> IPC mode, IPC namespace related <code>systemControls</code> will apply to all containers within a task.</p> </li> </ul> <note> <p>This parameter is not supported for Windows containers or tasks run on Fargate.</p> </note></p>
    #[serde(rename = "ipcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// <p><p>The amount of memory (in MiB) used by the task. It can be expressed as an integer using MiB (for example ,<code>1024</code>) or as a string using GB (for example, <code>1GB</code> or <code>1 GB</code>) in a task definition. String values are converted to an integer indicating the MiB when the task definition is registered.</p> <note> <p>Task-level CPU and memory parameters are ignored for Windows containers. We recommend specifying container-level resources for Windows containers.</p> </note> <p>If using the EC2 launch type, this field is optional.</p> <p>If using the Fargate launch type, this field is required and you must use one of the following values. This determines your range of supported values for the <code>cpu</code> parameter.</p> <p>The CPU units cannot be less than 1 vCPU when you use Windows containers on Fargate.</p> <ul> <li> <p>512 (0.5 GB), 1024 (1 GB), 2048 (2 GB) - Available <code>cpu</code> values: 256 (.25 vCPU)</p> </li> <li> <p>1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB) - Available <code>cpu</code> values: 512 (.5 vCPU)</p> </li> <li> <p>2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB) - Available <code>cpu</code> values: 1024 (1 vCPU)</p> </li> <li> <p>Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 2048 (2 vCPU)</p> </li> <li> <p>Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 4096 (4 vCPU)</p> </li> </ul></p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>The Docker networking mode to use for the containers in the task. The valid values are <code>none</code>, <code>bridge</code>, <code>awsvpc</code>, and <code>host</code>. If no network mode is specified, the default is <code>bridge</code>.</p> <p>For Amazon ECS tasks on Fargate, the <code>awsvpc</code> network mode is required. For Amazon ECS tasks on Amazon EC2 Linux instances, any network mode can be used. For Amazon ECS tasks on Amazon EC2 Windows instances, <code>&lt;default&gt;</code> or <code>awsvpc</code> can be used. If the network mode is set to <code>none</code>, you cannot specify port mappings in your container definitions, and the tasks containers do not have external connectivity. The <code>host</code> and <code>awsvpc</code> network modes offer the highest networking performance for containers because they use the EC2 network stack instead of the virtualized network stack provided by the <code>bridge</code> mode.</p> <p>With the <code>host</code> and <code>awsvpc</code> network modes, exposed container ports are mapped directly to the corresponding host port (for the <code>host</code> network mode) or the attached elastic network interface port (for the <code>awsvpc</code> network mode), so you cannot take advantage of dynamic host port mappings. </p> <important> <p>When using the <code>host</code> network mode, you should not run containers using the root user (UID 0). It is considered best practice to use a non-root user.</p> </important> <p>If the network mode is <code>awsvpc</code>, the task is allocated an elastic network interface, and you must specify a <a>NetworkConfiguration</a> value when you create a service or run a task with the task definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>If the network mode is <code>host</code>, you cannot run multiple instantiations of the same task on a single container instance when port mappings are used.</p> <p>For more information, see <a href="https://docs.docker.com/engine/reference/run/#network-settings">Network settings</a> in the <i>Docker run reference</i>.</p>
    #[serde(rename = "networkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// <p><p>The process namespace to use for the containers in the task. The valid values are <code>host</code> or <code>task</code>. If <code>host</code> is specified, then all containers within the tasks that specified the <code>host</code> PID mode on the same container instance share the same process namespace with the host Amazon EC2 instance. If <code>task</code> is specified, all containers within the specified task share the same process namespace. If no value is specified, the default is a private namespace. For more information, see <a href="https://docs.docker.com/engine/reference/run/#pid-settings---pid">PID settings</a> in the <i>Docker run reference</i>.</p> <p>If the <code>host</code> PID mode is used, be aware that there is a heightened risk of undesired process namespace expose. For more information, see <a href="https://docs.docker.com/engine/security/security/">Docker security</a>.</p> <note> <p>This parameter is not supported for Windows containers or tasks run on Fargate.</p> </note></p>
    #[serde(rename = "pidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// <p>An array of placement constraint objects to use for the task. You can specify a maximum of 10 constraints for each task. This limit includes constraints in the task definition and those specified at runtime.</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    /// <p>The configuration details for the App Mesh proxy.</p> <p>For tasks hosted on Amazon EC2 instances, the container instances require at least version <code>1.26.0</code> of the container agent and at least version <code>1.26.0-1</code> of the <code>ecs-init</code> package to use a proxy configuration. If your container instances are launched from the Amazon ECS-optimized AMI version <code>20190301</code> or later, then they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-ami-versions.html">Amazon ECS-optimized AMI versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "proxyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    /// <p>The task launch type that Amazon ECS validates the task definition against. A client exception is returned if the task definition doesn't validate against the compatibilities specified. If no value is specified, the parameter is omitted from the response.</p>
    #[serde(rename = "requiresCompatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    /// <p>The operating system that your tasks definitions run on. A platform family is specified only for tasks using the Fargate launch type. </p> <p>When you specify a task definition in a service, this value must match the <code>runtimePlatform</code> value of the service.</p>
    #[serde(rename = "runtimePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,
    /// <p><p>The metadata that you apply to the task definition to help you categorize and organize them. Each tag consists of a key and an optional value. You define both of them.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">IAM Roles for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "taskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    /// <p>A list of volume definitions in JSON format that containers in your task might use.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterTaskDefinitionResponse {
    /// <p>The list of tags associated with the task definition.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The full description of the registered task definition.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

/// <p>The repository credentials for private registry authentication.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RepositoryCredentials {
    /// <p><p>The Amazon Resource Name (ARN) of the secret containing the private repository credentials.</p> <note> <p>When you use the Amazon ECS API, CLI, or Amazon Web Services SDK, if the secret exists in the same Region as the task that you&#39;re launching then you can use either the full ARN or the name of the secret. When you use the Amazon Web Services Management Console, you must specify the full ARN of the secret.</p> </note></p>
    #[serde(rename = "credentialsParameter")]
    pub credentials_parameter: String,
}

/// <p>Describes the resources available for a container instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Resource {
    /// <p>When the <code>doubleValue</code> type is set, the value of the resource must be a double precision floating-point type.</p>
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    /// <p>When the <code>integerValue</code> type is set, the value of the resource must be an integer.</p>
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<i64>,
    /// <p>When the <code>longValue</code> type is set, the value of the resource must be an extended precision floating-point type.</p>
    #[serde(rename = "longValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    /// <p>The name of the resource, such as <code>CPU</code>, <code>MEMORY</code>, <code>PORTS</code>, <code>PORTS_UDP</code>, or a user-defined resource.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>When the <code>stringSetValue</code> type is set, the value of the resource must be a string type.</p>
    #[serde(rename = "stringSetValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_set_value: Option<Vec<String>>,
    /// <p>The type of the resource. Valid values: <code>INTEGER</code>, <code>DOUBLE</code>, <code>LONG</code>, or <code>STRINGSET</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The type and amount of a resource to assign to a container. The supported resource types are GPUs and Elastic Inference accelerators. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-gpu.html">Working with GPUs on Amazon ECS</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-inference.html">Working with Amazon Elastic Inference on Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i> </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceRequirement {
    /// <p>The type of resource to assign to a container. The supported values are <code>GPU</code> or <code>InferenceAccelerator</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>The value for the specified resource type.</p> <p>If the <code>GPU</code> type is used, the value is the number of physical <code>GPUs</code> the Amazon ECS container agent reserves for the container. The number of GPUs that's reserved for all containers in a task can't exceed the number of available GPUs on the container instance that the task is launched on.</p> <p>If the <code>InferenceAccelerator</code> type is used, the <code>value</code> matches the <code>deviceName</code> for an <a>InferenceAccelerator</a> specified in a task definition.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RunTaskRequest {
    /// <p>The capacity provider strategy to use for the task.</p> <p>If a <code>capacityProviderStrategy</code> is specified, the <code>launchType</code> parameter must be omitted. If no <code>capacityProviderStrategy</code> or <code>launchType</code> is specified, the <code>defaultCapacityProviderStrategy</code> for the cluster is used.</p> <p>When you use cluster auto scaling, you must specify <code>capacityProviderStrategy</code> and not <code>launchType</code>. </p> <p>A capacity provider strategy may contain a maximum of 6 capacity providers.</p>
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to run your task on. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The number of instantiations of the specified task to place on your cluster. You can specify up to 10 tasks for each call.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>Specifies whether to use Amazon ECS managed tags for the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>Determines whether to use the execute command functionality for the containers in this task. If <code>true</code>, this enables execute command functionality on all containers in the task.</p> <p>If <code>true</code>, then the task definition must have a task role, or you must provide one as an override.</p>
    #[serde(rename = "enableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    /// <p>The name of the task group to associate with the task. The default value is the family name of the task definition (for example, <code>family:my-family-name</code>).</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>The infrastructure to run your standalone task on. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS launch types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>The <code>FARGATE</code> launch type runs your tasks on Fargate On-Demand infrastructure.</p> <note> <p>Fargate Spot infrastructure is available for use but a capacity provider strategy must be used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/userguide/fargate-capacity-providers.html">Fargate capacity providers</a> in the <i>Amazon ECS User Guide for Fargate</i>.</p> </note> <p>The <code>EC2</code> launch type runs your tasks on Amazon EC2 instances registered to your cluster.</p> <p>The <code>EXTERNAL</code> launch type runs your tasks on your on-premises server or virtual machine (VM) capacity registered to your cluster.</p> <p>A task can use either a launch type or a capacity provider strategy. If a <code>launchType</code> is specified, the <code>capacityProviderStrategy</code> parameter must be omitted.</p> <p>When you use cluster auto scaling, you must specify <code>capacityProviderStrategy</code> and not <code>launchType</code>. </p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>The network configuration for the task. This parameter is required for task definitions that use the <code>awsvpc</code> network mode to receive their own elastic network interface, and it isn't supported for other network modes. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>A list of container overrides in JSON format that specify the name of a container in the specified task definition and the overrides it should receive. You can override the default command for a container (that's specified in the task definition or Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the task definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p> <p>A total of 8192 characters are allowed for overrides. This limit includes the JSON formatting characters of the override structure.</p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    /// <p>An array of placement constraint objects to use for the task. You can specify up to 10 constraints for each task (including constraints in the task definition and those specified at runtime).</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// <p>The placement strategy objects to use for the task. You can specify a maximum of 5 strategy rules for each task.</p>
    #[serde(rename = "placementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    /// <p>The platform version the task uses. A platform version is only specified for tasks hosted on Fargate. If one isn't specified, the <code>LATEST</code> platform version is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate platform versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p><p>Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags aren&#39;t propagated. Tags can only be propagated to the task during task creation. To add tags to a task after task creation, use the <a>TagResource</a> API action.</p> <note> <p>An error will be received if you specify the <code>SERVICE</code> option when running a task.</p> </note></p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>The reference ID to use for the task. The reference ID can have a maximum length of 1024 characters.</p>
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    /// <p>An optional tag specified when a task is started. For example, if you automatically trigger a task to run a batch process job, you could apply a unique identifier for that job to your task with the <code>startedBy</code> parameter. You can then identify which tasks belong to that job by filtering the results of a <a>ListTasks</a> call with the <code>startedBy</code> value. Up to 36 letters (uppercase and lowercase), numbers, hyphens (-), and underscores (_) are allowed.</p> <p>If a task is started by an Amazon ECS service, then the <code>startedBy</code> parameter contains the deployment ID of the service that starts it.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// <p><p>The metadata that you apply to the task to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full ARN of the task definition to run. If a <code>revision</code> isn't specified, the latest <code>ACTIVE</code> revision is used.</p> <p>When you create an IAM policy for run-task, you can set the resource to be the latest task definition revision, or a specific revision.</p> <p>The full ARN value must match the value that you specified as the <code>Resource</code> of the IAM principal's permissions policy.</p> <p>When you specify the policy resource as the latest task definition version (by setting the <code>Resource</code> in the policy to <code>arn:aws:ecs:us-east-1:111122223333:task-definition/TaskFamilyName</code>), then set this value to <code>arn:aws:ecs:us-east-1:111122223333:task-definition/TaskFamilyName</code>.</p> <p>When you specify the policy resource as a specific task definition version (by setting the <code>Resource</code> in the policy to <code>arn:aws:ecs:us-east-1:111122223333:task-definition/TaskFamilyName:1</code> or <code>arn:aws:ecs:us-east-1:111122223333:task-definition/TaskFamilyName:*</code>), then set this value to <code>arn:aws:ecs:us-east-1:111122223333:task-definition/TaskFamilyName:1</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/security_iam_service-with-iam.html#security_iam_service-with-iam-id-based-policies-resources">Policy Resources for Amazon ECS</a> in the Amazon Elastic Container Service developer Guide.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RunTaskResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>A full description of the tasks that were run. The tasks that were successfully placed on your cluster are described here.</p>
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

/// <p>Information about the platform for the Amazon ECS service or task.</p> <p>For more informataion about <code>RuntimePlatform</code>, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html#runtime-platform">RuntimePlatform</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RuntimePlatform {
    /// <p>The CPU architecture.</p> <p>You can run your Linux tasks on an ARM-based platform by setting the value to <code>ARM64</code>. This option is avaiable for tasks that run on Linux Amazon EC2 instance or Linux containers on Fargate.</p>
    #[serde(rename = "cpuArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_architecture: Option<String>,
    /// <p>The operating system.</p>
    #[serde(rename = "operatingSystemFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_family: Option<String>,
}

/// <p>A floating-point percentage of the desired number of tasks to place and keep running in the task set.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scale {
    /// <p>The unit of measure for the scale value.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>The value, specified as a percent total of a service's <code>desiredCount</code>, to scale the task set. Accepted values are numbers between 0 and 100.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>An object representing the secret to expose to your container. Secrets can be exposed to a container in the following ways:</p> <ul> <li> <p>To inject sensitive data into your containers as environment variables, use the <code>secrets</code> container definition parameter.</p> </li> <li> <p>To reference sensitive information in the log configuration of a container, use the <code>secretOptions</code> container definition parameter.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/specifying-sensitive-data.html">Specifying sensitive data</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Secret {
    /// <p>The name of the secret.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The secret to expose to the container. The supported values are either the full ARN of the Secrets Manager secret or the full ARN of the parameter in the SSM Parameter Store.</p> <p>For information about the require Identity and Access Management permissions, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/specifying-sensitive-data-secrets.html#secrets-iam">Required IAM permissions for Amazon ECS secrets</a> (for Secrets Manager) or <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/specifying-sensitive-data-parameters.html">Required IAM permissions for Amazon ECS secrets</a> (for Systems Manager Parameter store) in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>If the SSM Parameter Store parameter exists in the same Region as the task you&#39;re launching, then you can use either the full ARN or name of the parameter. If the parameter exists in a different Region, then the full ARN must be specified.</p> </note></p>
    #[serde(rename = "valueFrom")]
    pub value_from: String,
}

/// <p>Details on a service within a cluster</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Service {
    /// <p>The capacity provider strategy the service uses. When using the DescribeServices API, this field is omitted if the service was created using a launch type.</p>
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>The Amazon Resource Name (ARN) of the cluster that hosts the service.</p>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The Unix timestamp for the time when the service was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The principal that created the service.</p>
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>
    #[serde(rename = "deploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    /// <p>The deployment controller type the service is using. When using the DescribeServices API, this field is omitted if the service uses the <code>ECS</code> deployment controller type.</p>
    #[serde(rename = "deploymentController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<DeploymentController>,
    /// <p>The current state of deployments for the service.</p>
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
    /// <p>The desired number of instantiations of the task definition to keep running on the service. This value is specified when the service is created with <a>CreateService</a>, and it can be modified with <a>UpdateService</a>.</p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p>Determines whether to use Amazon ECS managed tags for the tasks in the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>Determines whether the execute command functionality is enabled for the service. If <code>true</code>, the execute command functionality is enabled for all containers in tasks as part of the service.</p>
    #[serde(rename = "enableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    /// <p>The event stream for your service. A maximum of 100 of the latest events are displayed.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ServiceEvent>>,
    /// <p>The period of time, in seconds, that the Amazon ECS service scheduler ignores unhealthy Elastic Load Balancing target health checks after a task has first started.</p>
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i64>,
    /// <p>The launch type the service is using. When using the DescribeServices API, this field is omitted if the service was created using a capacity provider strategy.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>A list of Elastic Load Balancing load balancer objects. It contains the load balancer name, the container name, and the container port to access from the load balancer. The container name is as it appears in a container definition.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>The VPC subnet and security group configuration for tasks that receive their own elastic network interface by using the <code>awsvpc</code> networking mode.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The number of tasks in the cluster that are in the <code>PENDING</code> state.</p>
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i64>,
    /// <p>The placement constraints for the tasks in the service.</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// <p>The placement strategy that determines how tasks for the service are placed.</p>
    #[serde(rename = "placementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    /// <p>The operating system that your tasks in the service run on. A platform family is specified only for tasks using the Fargate launch type. </p> <p> All tasks that run as part of this service must use the same <code>platformFamily</code> value as the service (for example, <code>LINUX</code>).</p>
    #[serde(rename = "platformFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    /// <p>The platform version to run your service on. A platform version is only specified for tasks that are hosted on Fargate. If one isn't specified, the <code>LATEST</code> platform version is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>Determines whether to propagate the tags from the task definition or the service to the task. If no value is specified, the tags aren't propagated.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>The ARN of the IAM role that's associated with the service. It allows the Amazon ECS container agent to register container instances with an Elastic Load Balancing load balancer.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The number of tasks in the cluster that are in the <code>RUNNING</code> state.</p>
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i64>,
    /// <p><p>The scheduling strategy to use for the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Services</a>.</p> <p>There are two service scheduler strategies available.</p> <ul> <li> <p> <code>REPLICA</code>-The replica scheduling strategy places and maintains the desired number of tasks across your cluster. By default, the service scheduler spreads tasks across Availability Zones. You can use task placement strategies and constraints to customize task placement decisions.</p> </li> <li> <p> <code>DAEMON</code>-The daemon scheduling strategy deploys exactly one task on each active container instance. This task meets all of the task placement constraints that you specify in your cluster. The service scheduler also evaluates the task placement constraints for running tasks. It stop tasks that don&#39;t meet the placement constraints.</p> <note> <p>Fargate tasks don&#39;t support the <code>DAEMON</code> scheduling strategy.</p> </note> </li> </ul></p>
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// <p>The ARN that identifies the service. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    #[serde(rename = "serviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    /// <p>The name of your service. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. Service names must be unique within a cluster. However, you can have similarly named services in multiple clusters within a Region or across multiple Regions.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The details for the service discovery registries to assign to this service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html">Service Discovery</a>.</p>
    #[serde(rename = "serviceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// <p>The status of the service. The valid values are <code>ACTIVE</code>, <code>DRAINING</code>, or <code>INACTIVE</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The metadata that you apply to the service to help you categorize and organize them. Each tag consists of a key and an optional value. You define bot the key and value.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The task definition to use for tasks in the service. This value is specified when the service is created with <a>CreateService</a>, and it can be modified with <a>UpdateService</a>.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    /// <p>Information about a set of Amazon ECS tasks in either an CodeDeploy or an <code>EXTERNAL</code> deployment. An Amazon ECS task set includes details such as the desired number of tasks, how many tasks are running, and whether the task set serves production traffic.</p>
    #[serde(rename = "taskSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets: Option<Vec<TaskSet>>,
}

/// <p>The details for an event that's associated with a service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceEvent {
    /// <p>The Unix timestamp for the time when the event was triggered.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID string for the event.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The event message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The details for the service registry.</p> <p>Each service may be associated with one service registry. Multiple service registries for each service are not supported.</p> <p>When you add, update, or remove the service registries configuration, Amazon ECS starts a new deployment. New tasks are registered and deregistered to the updated service registry configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceRegistry {
    /// <p>The container name value to be used for your service discovery service. It's already specified in the task definition. If the task definition that your service task specifies uses the <code>bridge</code> or <code>host</code> network mode, you must specify a <code>containerName</code> and <code>containerPort</code> combination from the task definition. If the task definition that your service task specifies uses the <code>awsvpc</code> network mode and a type SRV DNS record is used, you must specify either a <code>containerName</code> and <code>containerPort</code> combination or a <code>port</code> value. However, you can't specify both.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The port value to be used for your service discovery service. It's already specified in the task definition. If the task definition your service task specifies uses the <code>bridge</code> or <code>host</code> network mode, you must specify a <code>containerName</code> and <code>containerPort</code> combination from the task definition. If the task definition your service task specifies uses the <code>awsvpc</code> network mode and a type SRV DNS record is used, you must specify either a <code>containerName</code> and <code>containerPort</code> combination or a <code>port</code> value. However, you can't specify both.</p>
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The port value used if your service discovery service specified an SRV record. This field might be used if both the <code>awsvpc</code> network mode and SRV records are used.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the service registry. The currently supported service registry is Cloud Map. For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_CreateService.html">CreateService</a>.</p>
    #[serde(rename = "registryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
}

/// <p>The details for the execute command session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Session {
    /// <p>The ID of the execute command session.</p>
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// <p>A URL to the managed agent on the container that the SSM Session Manager client uses to send commands and receive output from the container.</p>
    #[serde(rename = "streamUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
    /// <p>An encrypted token value containing session and caller information. It's used to authenticate the connection to the container.</p>
    #[serde(rename = "tokenValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_value: Option<String>,
}

/// <p>The current account setting for a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Setting {
    /// <p>The Amazon ECS resource name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the principal. It can be an IAM user, IAM role, or the root user. If this field is omitted, the authenticated user is assumed.</p>
    #[serde(rename = "principalArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    /// <p>Determines whether the account setting is enabled or disabled for the specified resource.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartTaskRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster where to start your task. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance IDs or full ARN entries for the container instances where you would like to place your task. You can specify up to 10 container instances.</p>
    #[serde(rename = "containerInstances")]
    pub container_instances: Vec<String>,
    /// <p>Specifies whether to use Amazon ECS managed tags for the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>Whether or not the execute command functionality is enabled for the task. If <code>true</code>, this enables execute command functionality on all containers in the task.</p>
    #[serde(rename = "enableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    /// <p>The name of the task group to associate with the task. The default value is the family name of the task definition (for example, family:my-family-name).</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>The VPC subnet and security group configuration for tasks that receive their own elastic network interface by using the <code>awsvpc</code> networking mode.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p><p>A list of container overrides in JSON format that specify the name of a container in the specified task definition and the overrides it receives. You can override the default command for a container (that&#39;s specified in the task definition or Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the task definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p> <note> <p>A total of 8192 characters are allowed for overrides. This limit includes the JSON formatting characters of the override structure.</p> </note></p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    /// <p>Specifies whether to propagate the tags from the task definition or the service to the task. If no value is specified, the tags aren't propagated.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>The reference ID to use for the task.</p>
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    /// <p>An optional tag specified when a task is started. For example, if you automatically trigger a task to run a batch process job, you could apply a unique identifier for that job to your task with the <code>startedBy</code> parameter. You can then identify which tasks belong to that job by filtering the results of a <a>ListTasks</a> call with the <code>startedBy</code> value. Up to 36 letters (uppercase and lowercase), numbers, hyphens (-), and underscores (_) are allowed.</p> <p>If a task is started by an Amazon ECS service, the <code>startedBy</code> parameter contains the deployment ID of the service that starts it.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// <p><p>The metadata that you apply to the task to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full ARN of the task definition to start. If a <code>revision</code> isn't specified, the latest <code>ACTIVE</code> revision is used.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTaskResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>A full description of the tasks that were started. Each task that was successfully placed on your container instances is described.</p>
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopTaskRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task to stop. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>An optional message specified when a task is stopped. For example, if you're using a custom scheduler, you can use this parameter to specify the reason for stopping the task here, and the message appears in subsequent <a>DescribeTasks</a> API operations on this task. Up to 255 characters are allowed in this message.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The task ID or full Amazon Resource Name (ARN) of the task to stop.</p>
    #[serde(rename = "task")]
    pub task: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopTaskResponse {
    /// <p>The task that was stopped.</p>
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubmitAttachmentStateChangesRequest {
    /// <p>Any attachments associated with the state change request.</p>
    #[serde(rename = "attachments")]
    pub attachments: Vec<AttachmentStateChange>,
    /// <p>The short name or full ARN of the cluster that hosts the container instance the attachment belongs to.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubmitAttachmentStateChangesResponse {
    /// <p>Acknowledgement of the state change.</p>
    #[serde(rename = "acknowledgment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgment: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubmitContainerStateChangeRequest {
    /// <p>The short name or full ARN of the cluster that hosts the container.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The name of the container.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The exit code that's returned for the state change request.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The network bindings of the container.</p>
    #[serde(rename = "networkBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    /// <p>The reason for the state change request.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the Docker container.</p>
    #[serde(rename = "runtimeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,
    /// <p>The status of the state change request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The task ID or full Amazon Resource Name (ARN) of the task that hosts the container.</p>
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubmitContainerStateChangeResponse {
    /// <p>Acknowledgement of the state change.</p>
    #[serde(rename = "acknowledgment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgment: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubmitTaskStateChangeRequest {
    /// <p>Any attachments associated with the state change request.</p>
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentStateChange>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Any containers that's associated with the state change request.</p>
    #[serde(rename = "containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerStateChange>>,
    /// <p>The Unix timestamp for the time when the task execution stopped.</p>
    #[serde(rename = "executionStoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_stopped_at: Option<f64>,
    /// <p>The details for the managed agent that's associated with the task.</p>
    #[serde(rename = "managedAgents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_agents: Option<Vec<ManagedAgentStateChange>>,
    /// <p>The Unix timestamp for the time when the container image pull started.</p>
    #[serde(rename = "pullStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_started_at: Option<f64>,
    /// <p>The Unix timestamp for the time when the container image pull completed.</p>
    #[serde(rename = "pullStoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_stopped_at: Option<f64>,
    /// <p>The reason for the state change request.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The status of the state change request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The task ID or full ARN of the task in the state change request.</p>
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubmitTaskStateChangeResponse {
    /// <p>Acknowledgement of the state change.</p>
    #[serde(rename = "acknowledgment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgment: Option<String>,
}

/// <p><p>A list of namespaced kernel parameters to set in the container. This parameter maps to <code>Sysctls</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--sysctl</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p> <p>We don&#39;t recommend that you specify network-related <code>systemControls</code> parameters for multiple containers in a single task. This task also uses either the <code>awsvpc</code> or <code>host</code> network mode. It does it for the following reasons.</p> <ul> <li> <p>For tasks that use the <code>awsvpc</code> network mode, if you set <code>systemControls</code> for any container, it applies to all containers in the task. If you set different <code>systemControls</code> for multiple containers in a single task, the container that&#39;s started last determines which <code>systemControls</code> take effect.</p> </li> <li> <p>For tasks that use the <code>host</code> network mode, the <code>systemControls</code> parameter applies to the container instance&#39;s kernel parameter and that of all containers of any tasks running on that container instance.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SystemControl {
    /// <p>The namespaced kernel parameter to set a <code>value</code> for.</p>
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// <p>The value for the namespaced kernel parameter that's specified in <code>namespace</code>.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p><p>The metadata that you apply to a resource to help you categorize and organize them. Each tag consists of a key and an optional value. You define them.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that make up a tag. A <code>key</code> is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A <code>value</code> acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to add tags to. Currently, the supported resources are Amazon ECS capacity providers, tasks, services, task definitions, clusters, and container instances.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p><p>The tags to add to the resource. A tag is an array of key-value pairs.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Details on a task in a cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Task {
    /// <p>The Elastic Network Adapter that's associated with the task if the task uses the <code>awsvpc</code> network mode.</p>
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// <p>The attributes of the task</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The Availability Zone for the task.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The capacity provider that's associated with the task.</p>
    #[serde(rename = "capacityProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_name: Option<String>,
    /// <p>The ARN of the cluster that hosts the task.</p>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The connectivity status of a task.</p>
    #[serde(rename = "connectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<String>,
    /// <p>The Unix timestamp for the time when the task last went into <code>CONNECTED</code> status.</p>
    #[serde(rename = "connectivityAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_at: Option<f64>,
    /// <p>The ARN of the container instances that host the task.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The containers that's associated with the task.</p>
    #[serde(rename = "containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,
    /// <p><p>The number of CPU units used by the task as expressed in a task definition. It can be expressed as an integer using CPU units (for example, <code>1024</code>). It can also be expressed as a string using vCPUs (for example, <code>1 vCPU</code> or <code>1 vcpu</code>). String values are converted to an integer that indicates the CPU units when the task definition is registered.</p> <p>If you use the EC2 launch type, this field is optional. Supported values are between <code>128</code> CPU units (<code>0.125</code> vCPUs) and <code>10240</code> CPU units (<code>10</code> vCPUs).</p> <p>If you use the Fargate launch type, this field is required. You must use one of the following values. These values determine the range of supported values for the <code>memory</code> parameter:</p> <p>The CPU units cannot be less than 1 vCPU when you use Windows containers on Fargate.</p> <ul> <li> <p>256 (.25 vCPU) - Available <code>memory</code> values: 512 (0.5 GB), 1024 (1 GB), 2048 (2 GB)</p> </li> <li> <p>512 (.5 vCPU) - Available <code>memory</code> values: 1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB)</p> </li> <li> <p>1024 (1 vCPU) - Available <code>memory</code> values: 2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB)</p> </li> <li> <p>2048 (2 vCPU) - Available <code>memory</code> values: Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB)</p> </li> <li> <p>4096 (4 vCPU) - Available <code>memory</code> values: Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB)</p> </li> </ul></p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p>The Unix timestamp for the time when the task was created. More specifically, it's for the time when the task entered the <code>PENDING</code> state.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The desired status of the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-lifecycle.html">Task Lifecycle</a>.</p>
    #[serde(rename = "desiredStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<String>,
    /// <p>Determines whether execute command functionality is enabled for this task. If <code>true</code>, execute command functionality is enabled on all the containers in the task.</p>
    #[serde(rename = "enableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    /// <p>The ephemeral storage settings for the task.</p>
    #[serde(rename = "ephemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    /// <p>The Unix timestamp for the time when the task execution stopped.</p>
    #[serde(rename = "executionStoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_stopped_at: Option<f64>,
    /// <p>The name of the task group that's associated with the task.</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p><p>The health status for the task. It&#39;s determined by the health of the essential containers in the task. If all essential containers in the task are reporting as <code>HEALTHY</code>, the task status also reports as <code>HEALTHY</code>. If any essential containers in the task are reporting as <code>UNHEALTHY</code> or <code>UNKNOWN</code>, the task status also reports as <code>UNHEALTHY</code> or <code>UNKNOWN</code>.</p> <note> <p>The Amazon ECS container agent doesn&#39;t monitor or report on Docker health checks that are embedded in a container image and not specified in the container definition. For example, this includes those specified in a parent image or from the image&#39;s Dockerfile. Health check parameters that are specified in a container definition override any Docker health checks that are found in the container image.</p> </note></p>
    #[serde(rename = "healthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    /// <p>The Elastic Inference accelerator that's associated with the task.</p>
    #[serde(rename = "inferenceAccelerators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<InferenceAccelerator>>,
    /// <p>The last known status for the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-lifecycle.html">Task Lifecycle</a>.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The infrastructure where your task runs on. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS launch types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p><p>The amount of memory (in MiB) that the task uses as expressed in a task definition. It can be expressed as an integer using MiB (for example, <code>1024</code>). If it&#39;s expressed as a string using GB (for example, <code>1GB</code> or <code>1 GB</code>), it&#39;s converted to an integer indicating the MiB when the task definition is registered.</p> <p>If you use the EC2 launch type, this field is optional.</p> <p>If you use the Fargate launch type, this field is required. You must use one of the following values. The value that you choose determines the range of supported values for the <code>cpu</code> parameter.</p> <ul> <li> <p>512 (0.5 GB), 1024 (1 GB), 2048 (2 GB) - Available <code>cpu</code> values: 256 (.25 vCPU)</p> </li> <li> <p>1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB) - Available <code>cpu</code> values: 512 (.5 vCPU)</p> </li> <li> <p>2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB) - Available <code>cpu</code> values: 1024 (1 vCPU)</p> </li> <li> <p>Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 2048 (2 vCPU)</p> </li> <li> <p>Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 4096 (4 vCPU)</p> </li> </ul></p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>One or more container overrides.</p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    /// <p>The operating system that your tasks are running on. A platform family is specified only for tasks that use the Fargate launch type. </p> <p> All tasks that run as part of this service must use the same <code>platformFamily</code> value as the service (for example, <code>LINUX.</code>).</p>
    #[serde(rename = "platformFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    /// <p>The platform version where your task runs on. A platform version is only specified for tasks that use the Fargate launch type. If you didn't specify one, the <code>LATEST</code> platform version is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The Unix timestamp for the time when the container image pull began.</p>
    #[serde(rename = "pullStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_started_at: Option<f64>,
    /// <p>The Unix timestamp for the time when the container image pull completed.</p>
    #[serde(rename = "pullStoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_stopped_at: Option<f64>,
    /// <p>The Unix timestamp for the time when the task started. More specifically, it's for the time when the task transitioned from the <code>PENDING</code> state to the <code>RUNNING</code> state.</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p>The tag specified when a task is started. If an Amazon ECS service started the task, the <code>startedBy</code> parameter contains the deployment ID of that service.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// <p><p>The stop code indicating why a task was stopped. The <code>stoppedReason</code> might contain additional details.</p> <p>The following are valid values:</p> <ul> <li> <p> <code>TaskFailedToStart</code> </p> </li> <li> <p> <code>EssentialContainerExited</code> </p> </li> <li> <p> <code>UserInitiated</code> </p> </li> <li> <p> <code>TerminationNotice</code> </p> </li> <li> <p> <code>ServiceSchedulerInitiated</code> </p> </li> <li> <p> <code>SpotInterruption</code> </p> </li> </ul></p>
    #[serde(rename = "stopCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_code: Option<String>,
    /// <p>The Unix timestamp for the time when the task was stopped. More specifically, it's for the time when the task transitioned from the <code>RUNNING</code> state to the <code>STOPPED</code> state.</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<f64>,
    /// <p>The reason that the task was stopped.</p>
    #[serde(rename = "stoppedReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<String>,
    /// <p>The Unix timestamp for the time when the task stops. More specifically, it's for the time when the task transitions from the <code>RUNNING</code> state to <code>STOPPED</code>.</p>
    #[serde(rename = "stoppingAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_at: Option<f64>,
    /// <p><p>The metadata that you apply to the task to help you categorize and organize the task. Each tag consists of a key and an optional value. You define both the key and value.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the task.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The ARN of the task definition that creates the task.</p>
    #[serde(rename = "taskDefinitionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
    /// <p>The version counter for the task. Every time a task experiences a change that starts a CloudWatch event, the version counter is incremented. If you replicate your Amazon ECS task state with CloudWatch Events, you can compare the version of a task reported by the Amazon ECS API actions with the version reported in CloudWatch Events for the task (inside the <code>detail</code> object) to verify that the version in your event stream is current.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>The details of a task definition which describes the container and volume definitions of an Amazon Elastic Container Service task. You can specify which Docker images to use, the required resources, and other configurations related to launching the task definition through an Amazon ECS service or task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskDefinition {
    /// <p>The task launch types the task definition validated against during task definition registration. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS launch types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "compatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibilities: Option<Vec<String>>,
    /// <p>A list of container definitions in JSON format that describe the different containers that make up your task. For more information about container definition parameters and defaults, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Amazon ECS Task Definitions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "containerDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<Vec<ContainerDefinition>>,
    /// <p><p>The number of <code>cpu</code> units used by the task. If you use the EC2 launch type, this field is optional. Any value can be used. If you use the Fargate launch type, this field is required. You must use one of the following values. The value that you choose determines your range of valid values for the <code>memory</code> parameter.</p> <p>The CPU units cannot be less than 1 vCPU when you use Windows containers on Fargate.</p> <ul> <li> <p>256 (.25 vCPU) - Available <code>memory</code> values: 512 (0.5 GB), 1024 (1 GB), 2048 (2 GB)</p> </li> <li> <p>512 (.5 vCPU) - Available <code>memory</code> values: 1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB)</p> </li> <li> <p>1024 (1 vCPU) - Available <code>memory</code> values: 2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB)</p> </li> <li> <p>2048 (2 vCPU) - Available <code>memory</code> values: Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB)</p> </li> <li> <p>4096 (4 vCPU) - Available <code>memory</code> values: Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB)</p> </li> </ul></p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p>The Unix timestamp for the time when the task definition was deregistered.</p>
    #[serde(rename = "deregisteredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistered_at: Option<f64>,
    /// <p>The ephemeral storage settings to use for tasks run with the task definition.</p>
    #[serde(rename = "ephemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    /// <p>The Amazon Resource Name (ARN) of the task execution role that grants the Amazon ECS container agent permission to make Amazon Web Services API calls on your behalf. The task execution IAM role is required depending on the requirements of your task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_execution_IAM_role.html">Amazon ECS task execution IAM role</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The name of a family that this task definition is registered to. Up to 255 characters are allowed. Letters (both uppercase and lowercase letters), numbers, hyphens (-), and underscores (_) are allowed.</p> <p>A family groups multiple versions of a task definition. Amazon ECS gives the first task definition that you registered to a family a revision number of 1. Amazon ECS gives sequential revision numbers to each task definition that you add.</p>
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The Elastic Inference accelerator that's associated with the task.</p>
    #[serde(rename = "inferenceAccelerators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<InferenceAccelerator>>,
    /// <p><p>The IPC resource namespace to use for the containers in the task. The valid values are <code>host</code>, <code>task</code>, or <code>none</code>. If <code>host</code> is specified, then all containers within the tasks that specified the <code>host</code> IPC mode on the same container instance share the same IPC resources with the host Amazon EC2 instance. If <code>task</code> is specified, all containers within the specified task share the same IPC resources. If <code>none</code> is specified, then IPC resources within the containers of a task are private and not shared with other containers in a task or on the container instance. If no value is specified, then the IPC resource namespace sharing depends on the Docker daemon setting on the container instance. For more information, see <a href="https://docs.docker.com/engine/reference/run/#ipc-settings---ipc">IPC settings</a> in the <i>Docker run reference</i>.</p> <p>If the <code>host</code> IPC mode is used, be aware that there is a heightened risk of undesired IPC namespace expose. For more information, see <a href="https://docs.docker.com/engine/security/security/">Docker security</a>.</p> <p>If you are setting namespaced kernel parameters using <code>systemControls</code> for the containers in the task, the following will apply to your IPC resource namespace. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html">System Controls</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <ul> <li> <p>For tasks that use the <code>host</code> IPC mode, IPC namespace related <code>systemControls</code> are not supported.</p> </li> <li> <p>For tasks that use the <code>task</code> IPC mode, IPC namespace related <code>systemControls</code> will apply to all containers within a task.</p> </li> </ul> <note> <p>This parameter is not supported for Windows containers or tasks run on Fargate.</p> </note></p>
    #[serde(rename = "ipcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// <p><p>The amount (in MiB) of memory used by the task.</p> <p>If your tasks runs on Amazon EC2 instances, you must specify either a task-level memory value or a container-level memory value. This field is optional and any value can be used. If a task-level memory value is specified, the container-level memory value is optional. For more information regarding container-level memory and memory reservation, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ContainerDefinition.html">ContainerDefinition</a>.</p> <p>If your tasks runs on Fargate, this field is required. You must use one of the following values. The value you choose determines your range of valid values for the <code>cpu</code> parameter.</p> <ul> <li> <p>512 (0.5 GB), 1024 (1 GB), 2048 (2 GB) - Available <code>cpu</code> values: 256 (.25 vCPU)</p> </li> <li> <p>1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB) - Available <code>cpu</code> values: 512 (.5 vCPU)</p> </li> <li> <p>2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB) - Available <code>cpu</code> values: 1024 (1 vCPU)</p> </li> <li> <p>Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 2048 (2 vCPU)</p> </li> <li> <p>Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 4096 (4 vCPU)</p> </li> </ul></p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>The Docker networking mode to use for the containers in the task. The valid values are <code>none</code>, <code>bridge</code>, <code>awsvpc</code>, and <code>host</code>. If no network mode is specified, the default is <code>bridge</code>.</p> <p>For Amazon ECS tasks on Fargate, the <code>awsvpc</code> network mode is required. For Amazon ECS tasks on Amazon EC2 Linux instances, any network mode can be used. For Amazon ECS tasks on Amazon EC2 Windows instances, <code>&lt;default&gt;</code> or <code>awsvpc</code> can be used. If the network mode is set to <code>none</code>, you cannot specify port mappings in your container definitions, and the tasks containers do not have external connectivity. The <code>host</code> and <code>awsvpc</code> network modes offer the highest networking performance for containers because they use the EC2 network stack instead of the virtualized network stack provided by the <code>bridge</code> mode.</p> <p>With the <code>host</code> and <code>awsvpc</code> network modes, exposed container ports are mapped directly to the corresponding host port (for the <code>host</code> network mode) or the attached elastic network interface port (for the <code>awsvpc</code> network mode), so you cannot take advantage of dynamic host port mappings. </p> <important> <p>When using the <code>host</code> network mode, you should not run containers using the root user (UID 0). It is considered best practice to use a non-root user.</p> </important> <p>If the network mode is <code>awsvpc</code>, the task is allocated an elastic network interface, and you must specify a <a>NetworkConfiguration</a> value when you create a service or run a task with the task definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>If the network mode is <code>host</code>, you cannot run multiple instantiations of the same task on a single container instance when port mappings are used.</p> <p>For more information, see <a href="https://docs.docker.com/engine/reference/run/#network-settings">Network settings</a> in the <i>Docker run reference</i>.</p>
    #[serde(rename = "networkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// <p><p>The process namespace to use for the containers in the task. The valid values are <code>host</code> or <code>task</code>. If <code>host</code> is specified, then all containers within the tasks that specified the <code>host</code> PID mode on the same container instance share the same process namespace with the host Amazon EC2 instance. If <code>task</code> is specified, all containers within the specified task share the same process namespace. If no value is specified, the default is a private namespace. For more information, see <a href="https://docs.docker.com/engine/reference/run/#pid-settings---pid">PID settings</a> in the <i>Docker run reference</i>.</p> <p>If the <code>host</code> PID mode is used, be aware that there is a heightened risk of undesired process namespace expose. For more information, see <a href="https://docs.docker.com/engine/security/security/">Docker security</a>.</p> <note> <p>This parameter is not supported for Windows containers or tasks run on Fargate.</p> </note></p>
    #[serde(rename = "pidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// <p><p>An array of placement constraint objects to use for tasks.</p> <note> <p>This parameter isn&#39;t supported for tasks run on Fargate.</p> </note></p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    /// <p>The configuration details for the App Mesh proxy.</p> <p>Your Amazon ECS container instances require at least version 1.26.0 of the container agent and at least version 1.26.0-1 of the <code>ecs-init</code> package to use a proxy configuration. If your container instances are launched from the Amazon ECS optimized AMI version <code>20190301</code> or later, they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized Linux AMI</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "proxyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    /// <p>The Unix timestamp for the time when the task definition was registered.</p>
    #[serde(rename = "registeredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    /// <p>The principal that registered the task definition.</p>
    #[serde(rename = "registeredBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_by: Option<String>,
    /// <p><p>The container instance attributes required by your task. When an Amazon EC2 instance is registered to your cluster, the Amazon ECS container agent assigns some standard attributes to the instance. You can apply custom attributes. These are specified as key-value pairs using the Amazon ECS console or the <a>PutAttributes</a> API. These attributes are used when determining task placement for tasks hosted on Amazon EC2 instances. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes">Attributes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>This parameter isn&#39;t supported for tasks run on Fargate.</p> </note></p>
    #[serde(rename = "requiresAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_attributes: Option<Vec<Attribute>>,
    /// <p>The task launch types the task definition was validated against. To determine which task launch types the task definition is validated for, see the <a>TaskDefinition$compatibilities</a> parameter.</p>
    #[serde(rename = "requiresCompatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    /// <p>The revision of the task in a particular family. The revision is a version number of a task definition in a family. When you register a task definition for the first time, the revision is <code>1</code>. Each time that you register a new revision of a task definition in the same family, the revision value always increases by one. This is even if you deregistered previous revisions in this family.</p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    /// <p>The operating system that your task definitions are running on. A platform family is specified only for tasks using the Fargate launch type. </p> <p>When you specify a task in a service, this value must match the <code>runtimePlatform</code> value of the service.</p>
    #[serde(rename = "runtimePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,
    /// <p>The status of the task definition.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The full Amazon Resource Name (ARN) of the task definition.</p>
    #[serde(rename = "taskDefinitionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the Identity and Access Management role that grants containers in the task permission to call Amazon Web Services APIs on your behalf. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">Amazon ECS Task Role</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>IAM roles for tasks on Windows require that the <code>-EnableTaskIAMRole</code> option is set when you launch the Amazon ECS-optimized Windows AMI. Your containers must also run some configuration code to use the feature. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/windows_task_IAM_roles.html">Windows IAM roles for tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "taskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    /// <p><p>The list of data volume definitions for the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_data_volumes.html">Using data volumes in tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>The <code>host</code> and <code>sourcePath</code> parameters aren&#39;t supported for tasks run on Fargate.</p> </note></p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

/// <p><p>An object representing a constraint on task placement in the task definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html">Task placement constraints</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>Task placement constraints aren&#39;t supported for tasks run on Fargate.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TaskDefinitionPlacementConstraint {
    /// <p>A cluster query language expression to apply to the constraint. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster query language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The type of constraint. The <code>MemberOf</code> constraint restricts selection to be from a group of valid candidates.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The overrides that are associated with a task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TaskOverride {
    /// <p>One or more container overrides that are sent to a task.</p>
    #[serde(rename = "containerOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<Vec<ContainerOverride>>,
    /// <p>The cpu override for the task.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p><p>The ephemeral storage setting override for the task.</p> <note> <p>This parameter is only supported for tasks hosted on Fargate that use the following platform versions:</p> <ul> <li> <p>Linux platform version <code>1.4.0</code> or later.</p> </li> <li> <p>Windows platform version <code>1.0.0</code> or later.</p> </li> </ul> </note></p>
    #[serde(rename = "ephemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    /// <p>The Amazon Resource Name (ARN) of the task execution IAM role override for the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_execution_IAM_role.html">Amazon ECS task execution IAM role</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The Elastic Inference accelerator override for the task.</p>
    #[serde(rename = "inferenceAcceleratorOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerator_overrides: Option<Vec<InferenceAcceleratorOverride>>,
    /// <p>The memory override for the task.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">IAM Role for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "taskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
}

/// <p>Information about a set of Amazon ECS tasks in either an CodeDeploy or an <code>EXTERNAL</code> deployment. An Amazon ECS task set includes details such as the desired number of tasks, how many tasks are running, and whether the task set serves production traffic.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskSet {
    /// <p>The capacity provider strategy that are associated with the task set.</p>
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>The Amazon Resource Name (ARN) of the cluster that the service that hosts the task set exists in.</p>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The computed desired count for the task set. This is calculated by multiplying the service's <code>desiredCount</code> by the task set's <code>scale</code> percentage. The result is always rounded up. For example, if the computed desired count is 1.2, it rounds up to 2 tasks.</p>
    #[serde(rename = "computedDesiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_desired_count: Option<i64>,
    /// <p>The Unix timestamp for the time when the task set was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The external ID associated with the task set.</p> <p>If an CodeDeploy deployment created a task set, the <code>externalId</code> parameter contains the CodeDeploy deployment ID.</p> <p>If a task set is created for an external deployment and is associated with a service discovery registry, the <code>externalId</code> parameter contains the <code>ECS_TASK_SET_EXTERNAL_ID</code> Cloud Map attribute.</p>
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The ID of the task set.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The launch type the tasks in the task set are using. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS launch types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>Details on a load balancer that are used with a task set.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>The network configuration for the task set.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The number of tasks in the task set that are in the <code>PENDING</code> status during a deployment. A task in the <code>PENDING</code> state is preparing to enter the <code>RUNNING</code> state. A task set enters the <code>PENDING</code> status when it launches for the first time or when it's restarted after being in the <code>STOPPED</code> state.</p>
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i64>,
    /// <p>The operating system that your tasks in the set are running on. A platform family is specified only for tasks that use the Fargate launch type. </p> <p> All tasks in the set must have the same value.</p>
    #[serde(rename = "platformFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    /// <p>The Fargate platform version where the tasks in the task set are running. A platform version is only specified for tasks run on Fargate. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate platform versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The number of tasks in the task set that are in the <code>RUNNING</code> status during a deployment. A task in the <code>RUNNING</code> state is running and ready for use.</p>
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i64>,
    /// <p>A floating-point percentage of your desired number of tasks to place and keep running in the task set.</p>
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    /// <p>The Amazon Resource Name (ARN) of the service the task set exists in.</p>
    #[serde(rename = "serviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    /// <p>The details for the service discovery registries to assign to this task set. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html">Service discovery</a>.</p>
    #[serde(rename = "serviceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// <p>The stability status. This indicates whether the task set has reached a steady state. If the following conditions are met, the task set sre in <code>STEADY_STATE</code>:</p> <ul> <li> <p>The task <code>runningCount</code> is equal to the <code>computedDesiredCount</code>.</p> </li> <li> <p>The <code>pendingCount</code> is <code>0</code>.</p> </li> <li> <p>There are no tasks that are running on container instances in the <code>DRAINING</code> status.</p> </li> <li> <p>All tasks are reporting a healthy status from the load balancers, service discovery, and container health checks.</p> </li> </ul> <p>If any of those conditions aren't met, the stability status returns <code>STABILIZING</code>.</p>
    #[serde(rename = "stabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability_status: Option<String>,
    /// <p>The Unix timestamp for the time when the task set stability status was retrieved.</p>
    #[serde(rename = "stabilityStatusAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability_status_at: Option<f64>,
    /// <p>The tag specified when a task set is started. If an CodeDeploy deployment created the task set, the <code>startedBy</code> parameter is <code>CODE_DEPLOY</code>. If an external deployment created the task set, the startedBy field isn't used.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// <p><p>The status of the task set. The following describes each state.</p> <dl> <dt>PRIMARY</dt> <dd> <p>The task set is serving production traffic.</p> </dd> <dt>ACTIVE</dt> <dd> <p>The task set isn&#39;t serving production traffic.</p> </dd> <dt>DRAINING</dt> <dd> <p>The tasks in the task set are being stopped, and their corresponding targets are being deregistered from their target group.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The metadata that you apply to the task set to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case-sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The task definition that the task set is using.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task set.</p>
    #[serde(rename = "taskSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set_arn: Option<String>,
    /// <p>The Unix timestamp for the time when the task set was last updated.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The container path, mount options, and size of the tmpfs mount.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tmpfs {
    /// <p>The absolute file path where the tmpfs volume is to be mounted.</p>
    #[serde(rename = "containerPath")]
    pub container_path: String,
    /// <p>The list of tmpfs volume mount options.</p> <p>Valid values: <code>"defaults" | "ro" | "rw" | "suid" | "nosuid" | "dev" | "nodev" | "exec" | "noexec" | "sync" | "async" | "dirsync" | "remount" | "mand" | "nomand" | "atime" | "noatime" | "diratime" | "nodiratime" | "bind" | "rbind" | "unbindable" | "runbindable" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave" | "relatime" | "norelatime" | "strictatime" | "nostrictatime" | "mode" | "uid" | "gid" | "nr_inodes" | "nr_blocks" | "mpol"</code> </p>
    #[serde(rename = "mountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,
    /// <p>The maximum size (in MiB) of the tmpfs volume.</p>
    #[serde(rename = "size")]
    pub size: i64,
}

/// <p>The <code>ulimit</code> settings to pass to the container.</p> <p>Amazon ECS tasks hosted on Fargate use the default resource limit values set by the operating system with the exception of the <code>nofile</code> resource limit parameter which Fargate overrides. The <code>nofile</code> resource limit sets a restriction on the number of open files that a container can use. The default <code>nofile</code> soft limit is <code>1024</code> and hard limit is <code>4096</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ulimit {
    /// <p>The hard limit for the ulimit type.</p>
    #[serde(rename = "hardLimit")]
    pub hard_limit: i64,
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The soft limit for the ulimit type.</p>
    #[serde(rename = "softLimit")]
    pub soft_limit: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to delete tags from. Currently, the supported resources are Amazon ECS capacity providers, tasks, services, task definitions, clusters, and container instances.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCapacityProviderRequest {
    /// <p>An object that represent the parameters to update for the Auto Scaling group capacity provider.</p>
    #[serde(rename = "autoScalingGroupProvider")]
    pub auto_scaling_group_provider: AutoScalingGroupProviderUpdate,
    /// <p>The name of the capacity provider to update.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCapacityProviderResponse {
    /// <p>Details about the capacity provider.</p>
    #[serde(rename = "capacityProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateClusterRequest {
    /// <p>The name of the cluster to modify the settings for.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// <p>The execute command configuration for the cluster.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ClusterConfiguration>,
    /// <p>The cluster settings for your cluster.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<ClusterSetting>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterResponse {
    /// <p>Details about the cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateClusterSettingsRequest {
    /// <p>The name of the cluster to modify the settings for.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// <p>The setting to use by default for a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <a>PutAccountSetting</a> or <a>PutAccountSettingDefault</a>.</p>
    #[serde(rename = "settings")]
    pub settings: Vec<ClusterSetting>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterSettingsResponse {
    /// <p>Details about the cluster</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateContainerAgentRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that your container instance is running on. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance ID or full ARN entries for the container instance where you would like to update the Amazon ECS container agent.</p>
    #[serde(rename = "containerInstance")]
    pub container_instance: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateContainerAgentResponse {
    /// <p>The container instance that the container agent was updated for.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateContainerInstancesStateRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instance to update. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>A list of up to 10 container instance IDs or full ARN entries.</p>
    #[serde(rename = "containerInstances")]
    pub container_instances: Vec<String>,
    /// <p>The container instance state to update the container instance with. The only valid values for this action are <code>ACTIVE</code> and <code>DRAINING</code>. A container instance can only be updated to <code>DRAINING</code> status once it has reached an <code>ACTIVE</code> state. If a container instance is in <code>REGISTERING</code>, <code>DEREGISTERING</code>, or <code>REGISTRATION_FAILED</code> state you can describe the container instance but can't update the container instance state.</p>
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateContainerInstancesStateResponse {
    /// <p>The list of container instances.</p>
    #[serde(rename = "containerInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instances: Option<Vec<ContainerInstance>>,
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateServicePrimaryTaskSetRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set exists in.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// <p>The short name or full Amazon Resource Name (ARN) of the task set to set as the primary task set in the deployment.</p>
    #[serde(rename = "primaryTaskSet")]
    pub primary_task_set: String,
    /// <p>The short name or full Amazon Resource Name (ARN) of the service that the task set exists in.</p>
    #[serde(rename = "service")]
    pub service: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServicePrimaryTaskSetResponse {
    /// <p>etails about the task set.</p>
    #[serde(rename = "taskSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set: Option<TaskSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateServiceRequest {
    /// <p><p>The capacity provider strategy to update the service to use.</p> <p>if the service uses the default capacity provider strategy for the cluster, the service can be updated to use one or more capacity providers as opposed to the default capacity provider strategy. However, when a service is using a capacity provider strategy that&#39;s not the default capacity provider strategy, the service can&#39;t be updated to use the cluster&#39;s default capacity provider strategy.</p> <p>A capacity provider strategy consists of one or more capacity providers along with the <code>base</code> and <code>weight</code> to assign to them. A capacity provider must be associated with the cluster to be used in a capacity provider strategy. The <a>PutClusterCapacityProviders</a> API is used to associate a capacity provider with a cluster. Only capacity providers with an <code>ACTIVE</code> or <code>UPDATING</code> status can be used.</p> <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must already be created. New capacity providers can be created with the <a>CreateCapacityProvider</a> API operation.</p> <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p> <p>The <a>PutClusterCapacityProviders</a> API operation is used to update the list of available capacity providers for a cluster after the cluster is created.</p> <p/></p>
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that your service runs on. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>
    #[serde(rename = "deploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    /// <p>The number of instantiations of the task to place and keep running in your service.</p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p>Determines whether to turn on Amazon ECS managed tags for the tasks in the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>Only tasks launched after the update will reflect the update. To update the tags on all tasks, set <code>forceNewDeployment</code> to <code>true</code>, so that Amazon ECS starts new tasks with the updated tags.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>If <code>true</code>, this enables execute command functionality on all task containers.</p> <p>If you do not want to override the value that was set when the service was created, you can set this to <code>null</code> when performing this action.</p>
    #[serde(rename = "enableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    /// <p>Determines whether to force a new deployment of the service. By default, deployments aren't forced. You can use this option to start a new deployment with no service definition changes. For example, you can update a service's tasks to use a newer Docker image with the same image/tag combination (<code>my_image:latest</code>) or to roll Fargate tasks onto a newer platform version.</p>
    #[serde(rename = "forceNewDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_new_deployment: Option<bool>,
    /// <p>The period of time, in seconds, that the Amazon ECS service scheduler ignores unhealthy Elastic Load Balancing target health checks after a task has first started. This is only valid if your service is configured to use a load balancer. If your service's tasks take a while to start and respond to Elastic Load Balancing health checks, you can specify a health check grace period of up to 2,147,483,647 seconds. During that time, the Amazon ECS service scheduler ignores the Elastic Load Balancing health check status. This grace period can prevent the ECS service scheduler from marking tasks as unhealthy and stopping them before they have time to come up.</p>
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i64>,
    /// <p>A list of Elastic Load Balancing load balancer objects. It contains the load balancer name, the container name, and the container port to access from the load balancer. The container name is as it appears in a container definition.</p> <p>When you add, update, or remove a load balancer configuration, Amazon ECS starts new tasks with the updated Elastic Load Balancing configuration, and then stops the old tasks when the new tasks are running.</p> <p>For services that use rolling updates, you can add, update, or remove Elastic Load Balancing target groups. You can update from a single target group to multiple target groups and from multiple target groups to a single target group.</p> <p>For services that use blue/green deployments, you can update Elastic Load Balancing target groups by using <code> <a href="https://docs.aws.amazon.com/codedeploy/latest/APIReference/API_CreateDeployment.html">CreateDeployment</a> </code> through CodeDeploy. Note that multiple target groups are not supported for blue/green deployments. For more information see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/register-multiple-targetgroups.html">Register multiple target groups with a service</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. </p> <p>For services that use the external deployment controller, you can add, update, or remove load balancers by using <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateTaskSet.html">CreateTaskSet</a>. Note that multiple target groups are not supported for external deployments. For more information see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/register-multiple-targetgroups.html">Register multiple target groups with a service</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. </p> <p>You can remove existing <code>loadBalancers</code> by passing an empty list.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>An object representing the network configuration for the service.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>An array of task placement constraint objects to update the service to use. If no value is specified, the existing placement constraints for the service will remain unchanged. If this value is specified, it will override any existing placement constraints defined for the service. To remove all existing placement constraints, specify an empty array.</p> <p>You can specify a maximum of 10 constraints for each task. This limit includes constraints in the task definition and those specified at runtime.</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// <p>The task placement strategy objects to update the service to use. If no value is specified, the existing placement strategy for the service will remain unchanged. If this value is specified, it will override the existing placement strategy defined for the service. To remove an existing placement strategy, specify an empty object.</p> <p>You can specify a maximum of five strategy rules for each service.</p>
    #[serde(rename = "placementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    /// <p>The platform version that your tasks in the service run on. A platform version is only specified for tasks using the Fargate launch type. If a platform version is not specified, the <code>LATEST</code> platform version is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>Determines whether to propagate the tags from the task definition or the service to the task. If no value is specified, the tags aren't propagated.</p> <p>Only tasks launched after the update will reflect the update. To update the tags on all tasks, set <code>forceNewDeployment</code> to <code>true</code>, so that Amazon ECS starts new tasks with the updated tags.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>The name of the service to update.</p>
    #[serde(rename = "service")]
    pub service: String,
    /// <p>The details for the service discovery registries to assign to this service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html">Service Discovery</a>.</p> <p>When you add, update, or remove the service registries configuration, Amazon ECS starts new tasks with the updated service registries configuration, and then stops the old tasks when the new tasks are running.</p> <p>You can remove existing <code>serviceRegistries</code> by passing an empty list.</p>
    #[serde(rename = "serviceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full ARN of the task definition to run in your service. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used. If you modify the task definition with <code>UpdateService</code>, Amazon ECS spawns a task with the new version of the task definition and then stops an old task after the new version is running.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServiceResponse {
    /// <p>The full description of your service following the update call.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTaskSetRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set is found in.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// <p>A floating-point percentage of the desired number of tasks to place and keep running in the task set.</p>
    #[serde(rename = "scale")]
    pub scale: Scale,
    /// <p>The short name or full Amazon Resource Name (ARN) of the service that the task set is found in.</p>
    #[serde(rename = "service")]
    pub service: String,
    /// <p>The short name or full Amazon Resource Name (ARN) of the task set to update.</p>
    #[serde(rename = "taskSet")]
    pub task_set: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTaskSetResponse {
    /// <p>Details about the task set.</p>
    #[serde(rename = "taskSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set: Option<TaskSet>,
}

/// <p>The Docker and Amazon ECS container agent version information about a container instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VersionInfo {
    /// <p>The Git commit hash for the Amazon ECS container agent build on the <a href="https://github.com/aws/amazon-ecs-agent/commits/master">amazon-ecs-agent </a> GitHub repository.</p>
    #[serde(rename = "agentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_hash: Option<String>,
    /// <p>The version number of the Amazon ECS container agent.</p>
    #[serde(rename = "agentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>The Docker version that's running on the container instance.</p>
    #[serde(rename = "dockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
}

/// <p>A data volume that's used in a task definition. For tasks that use the Amazon Elastic File System (Amazon EFS), specify an <code>efsVolumeConfiguration</code>. For Windows tasks that use Amazon FSx for Windows File Server file system, specify a <code>fsxWindowsFileServerVolumeConfiguration</code>. For tasks that use a Docker volume, specify a <code>DockerVolumeConfiguration</code>. For tasks that use a bind mount host volume, specify a <code>host</code> and optional <code>sourcePath</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_data_volumes.html">Using Data Volumes in Tasks</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Volume {
    /// <p><p>This parameter is specified when you use Docker volumes.</p> <p>Windows containers only support the use of the <code>local</code> driver. To use bind mounts, specify the <code>host</code> parameter instead.</p> <note> <p>Docker volumes aren&#39;t supported by tasks run on Fargate.</p> </note></p>
    #[serde(rename = "dockerVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_volume_configuration: Option<DockerVolumeConfiguration>,
    /// <p>This parameter is specified when you use an Amazon Elastic File System file system for task storage.</p>
    #[serde(rename = "efsVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_volume_configuration: Option<EFSVolumeConfiguration>,
    /// <p>This parameter is specified when you use Amazon FSx for Windows File Server file system for task storage.</p>
    #[serde(rename = "fsxWindowsFileServerVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsx_windows_file_server_volume_configuration:
        Option<FSxWindowsFileServerVolumeConfiguration>,
    /// <p>This parameter is specified when you use bind mount host volumes. The contents of the <code>host</code> parameter determine whether your bind mount host volume persists on the host container instance and where it's stored. If the <code>host</code> parameter is empty, then the Docker daemon assigns a host path for your data volume. However, the data isn't guaranteed to persist after the containers that are associated with it stop running.</p> <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives. For example, you can mount <code>C:\my\path:C:\my\path</code> and <code>D:\:D:\</code>, but not <code>D:\my\path:C:\my\path</code> or <code>D:\:C:\my\path</code>.</p>
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<HostVolumeProperties>,
    /// <p>The name of the volume. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. This name is referenced in the <code>sourceVolume</code> parameter of container definition <code>mountPoints</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Details on a data volume from another container in the same task definition.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VolumeFrom {
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. If this value is <code>false</code>, then the container can write to the volume. The default value is <code>false</code>.</p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of another container within the same task definition to mount volumes from.</p>
    #[serde(rename = "sourceContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_container: Option<String>,
}

/// Errors returned by CreateCapacityProvider
#[derive(Debug, PartialEq)]
pub enum CreateCapacityProviderError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The limit for the resource was exceeded.</p>
    LimitExceeded(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>There's already a current Amazon ECS container agent update in progress on the container instance that's specified. If the container agent becomes disconnected while it's in a transitional stage, such as <code>PENDING</code> or <code>STAGING</code>, the update process can get stuck in that state. However, when the agent reconnects, it resumes where it stopped previously.</p>
    UpdateInProgress(String),
}

impl CreateCapacityProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCapacityProviderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateCapacityProviderError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateCapacityProviderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateCapacityProviderError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateCapacityProviderError::Server(err.msg))
                }
                "UpdateInProgressException" => {
                    return RusotoError::Service(CreateCapacityProviderError::UpdateInProgress(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCapacityProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCapacityProviderError::Client(ref cause) => write!(f, "{}", cause),
            CreateCapacityProviderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateCapacityProviderError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateCapacityProviderError::Server(ref cause) => write!(f, "{}", cause),
            CreateCapacityProviderError::UpdateInProgress(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCapacityProviderError {}
/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateClusterError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateClusterError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateClusterError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateClusterError::Client(ref cause) => write!(f, "{}", cause),
            CreateClusterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateClusterError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateClusterError {}
/// Errors returned by CreateService
#[derive(Debug, PartialEq)]
pub enum CreateServiceError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified platform version doesn't satisfy the required capabilities of the task definition.</p>
    PlatformTaskDefinitionIncompatibility(String),
    /// <p>The specified platform version doesn't exist.</p>
    PlatformUnknown(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified task isn't supported in this Region.</p>
    UnsupportedFeature(String),
}

impl CreateServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateServiceError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateServiceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(CreateServiceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateServiceError::InvalidParameter(err.msg))
                }
                "PlatformTaskDefinitionIncompatibilityException" => {
                    return RusotoError::Service(
                        CreateServiceError::PlatformTaskDefinitionIncompatibility(err.msg),
                    )
                }
                "PlatformUnknownException" => {
                    return RusotoError::Service(CreateServiceError::PlatformUnknown(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateServiceError::Server(err.msg))
                }
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(CreateServiceError::UnsupportedFeature(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateServiceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateServiceError::Client(ref cause) => write!(f, "{}", cause),
            CreateServiceError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            CreateServiceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateServiceError::PlatformTaskDefinitionIncompatibility(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateServiceError::PlatformUnknown(ref cause) => write!(f, "{}", cause),
            CreateServiceError::Server(ref cause) => write!(f, "{}", cause),
            CreateServiceError::UnsupportedFeature(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateServiceError {}
/// Errors returned by CreateTaskSet
#[derive(Debug, PartialEq)]
pub enum CreateTaskSetError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified platform version doesn't satisfy the required capabilities of the task definition.</p>
    PlatformTaskDefinitionIncompatibility(String),
    /// <p>The specified platform version doesn't exist.</p>
    PlatformUnknown(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service isn't active. You can't update a service that's inactive. If you have previously deleted a service, you can re-create it with <a>CreateService</a>.</p>
    ServiceNotActive(String),
    /// <p>The specified service wasn't found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster specific and Region specific.</p>
    ServiceNotFound(String),
    /// <p>The specified task isn't supported in this Region.</p>
    UnsupportedFeature(String),
}

impl CreateTaskSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTaskSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateTaskSetError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateTaskSetError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(CreateTaskSetError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateTaskSetError::InvalidParameter(err.msg))
                }
                "PlatformTaskDefinitionIncompatibilityException" => {
                    return RusotoError::Service(
                        CreateTaskSetError::PlatformTaskDefinitionIncompatibility(err.msg),
                    )
                }
                "PlatformUnknownException" => {
                    return RusotoError::Service(CreateTaskSetError::PlatformUnknown(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateTaskSetError::Server(err.msg))
                }
                "ServiceNotActiveException" => {
                    return RusotoError::Service(CreateTaskSetError::ServiceNotActive(err.msg))
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(CreateTaskSetError::ServiceNotFound(err.msg))
                }
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(CreateTaskSetError::UnsupportedFeature(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTaskSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTaskSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateTaskSetError::Client(ref cause) => write!(f, "{}", cause),
            CreateTaskSetError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            CreateTaskSetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateTaskSetError::PlatformTaskDefinitionIncompatibility(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTaskSetError::PlatformUnknown(ref cause) => write!(f, "{}", cause),
            CreateTaskSetError::Server(ref cause) => write!(f, "{}", cause),
            CreateTaskSetError::ServiceNotActive(ref cause) => write!(f, "{}", cause),
            CreateTaskSetError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
            CreateTaskSetError::UnsupportedFeature(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTaskSetError {}
/// Errors returned by DeleteAccountSetting
#[derive(Debug, PartialEq)]
pub enum DeleteAccountSettingError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeleteAccountSettingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAccountSettingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteAccountSettingError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAccountSettingError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteAccountSettingError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAccountSettingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAccountSettingError::Client(ref cause) => write!(f, "{}", cause),
            DeleteAccountSettingError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteAccountSettingError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAccountSettingError {}
/// Errors returned by DeleteAttributes
#[derive(Debug, PartialEq)]
pub enum DeleteAttributesError {
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified target wasn't found. You can view your available container instances with <a>ListContainerInstances</a>. Amazon ECS container instances are cluster-specific and Region-specific.</p>
    TargetNotFound(String),
}

impl DeleteAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeleteAttributesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAttributesError::InvalidParameter(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(DeleteAttributesError::TargetNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAttributesError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteAttributesError::TargetNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAttributesError {}
/// Errors returned by DeleteCapacityProvider
#[derive(Debug, PartialEq)]
pub enum DeleteCapacityProviderError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeleteCapacityProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCapacityProviderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteCapacityProviderError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteCapacityProviderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteCapacityProviderError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCapacityProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCapacityProviderError::Client(ref cause) => write!(f, "{}", cause),
            DeleteCapacityProviderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteCapacityProviderError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCapacityProviderError {}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>You can't delete a cluster that has registered container instances. First, deregister the container instances before you can delete the cluster. For more information, see <a>DeregisterContainerInstance</a>.</p>
    ClusterContainsContainerInstances(String),
    /// <p>You can't delete a cluster that contains services. First, update the service to reduce its desired task count to 0, and then delete the service. For more information, see <a>UpdateService</a> and <a>DeleteService</a>.</p>
    ClusterContainsServices(String),
    /// <p>You can't delete a cluster that has active tasks.</p>
    ClusterContainsTasks(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>There's already a current Amazon ECS container agent update in progress on the container instance that's specified. If the container agent becomes disconnected while it's in a transitional stage, such as <code>PENDING</code> or <code>STAGING</code>, the update process can get stuck in that state. However, when the agent reconnects, it resumes where it stopped previously.</p>
    UpdateInProgress(String),
}

impl DeleteClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteClusterError::Client(err.msg))
                }
                "ClusterContainsContainerInstancesException" => {
                    return RusotoError::Service(
                        DeleteClusterError::ClusterContainsContainerInstances(err.msg),
                    )
                }
                "ClusterContainsServicesException" => {
                    return RusotoError::Service(DeleteClusterError::ClusterContainsServices(
                        err.msg,
                    ))
                }
                "ClusterContainsTasksException" => {
                    return RusotoError::Service(DeleteClusterError::ClusterContainsTasks(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeleteClusterError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteClusterError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteClusterError::Server(err.msg))
                }
                "UpdateInProgressException" => {
                    return RusotoError::Service(DeleteClusterError::UpdateInProgress(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteClusterError::Client(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::ClusterContainsContainerInstances(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteClusterError::ClusterContainsServices(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::ClusterContainsTasks(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::Server(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::UpdateInProgress(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteClusterError {}
/// Errors returned by DeleteService
#[derive(Debug, PartialEq)]
pub enum DeleteServiceError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service wasn't found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster specific and Region specific.</p>
    ServiceNotFound(String),
}

impl DeleteServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteServiceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeleteServiceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteServiceError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteServiceError::Server(err.msg))
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(DeleteServiceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteServiceError::Client(ref cause) => write!(f, "{}", cause),
            DeleteServiceError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DeleteServiceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteServiceError::Server(ref cause) => write!(f, "{}", cause),
            DeleteServiceError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteServiceError {}
/// Errors returned by DeleteTaskSet
#[derive(Debug, PartialEq)]
pub enum DeleteTaskSetError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service isn't active. You can't update a service that's inactive. If you have previously deleted a service, you can re-create it with <a>CreateService</a>.</p>
    ServiceNotActive(String),
    /// <p>The specified service wasn't found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster specific and Region specific.</p>
    ServiceNotFound(String),
    /// <p>The specified task set wasn't found. You can view your available task sets with <a>DescribeTaskSets</a>. Task sets are specific to each cluster, service and Region.</p>
    TaskSetNotFound(String),
    /// <p>The specified task isn't supported in this Region.</p>
    UnsupportedFeature(String),
}

impl DeleteTaskSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTaskSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteTaskSetError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(DeleteTaskSetError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeleteTaskSetError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteTaskSetError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteTaskSetError::Server(err.msg))
                }
                "ServiceNotActiveException" => {
                    return RusotoError::Service(DeleteTaskSetError::ServiceNotActive(err.msg))
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(DeleteTaskSetError::ServiceNotFound(err.msg))
                }
                "TaskSetNotFoundException" => {
                    return RusotoError::Service(DeleteTaskSetError::TaskSetNotFound(err.msg))
                }
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(DeleteTaskSetError::UnsupportedFeature(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTaskSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTaskSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteTaskSetError::Client(ref cause) => write!(f, "{}", cause),
            DeleteTaskSetError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTaskSetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteTaskSetError::Server(ref cause) => write!(f, "{}", cause),
            DeleteTaskSetError::ServiceNotActive(ref cause) => write!(f, "{}", cause),
            DeleteTaskSetError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTaskSetError::TaskSetNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTaskSetError::UnsupportedFeature(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTaskSetError {}
/// Errors returned by DeregisterContainerInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterContainerInstanceError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeregisterContainerInstanceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterContainerInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeregisterContainerInstanceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeregisterContainerInstanceError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeregisterContainerInstanceError::InvalidParameter(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(DeregisterContainerInstanceError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterContainerInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterContainerInstanceError::Client(ref cause) => write!(f, "{}", cause),
            DeregisterContainerInstanceError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DeregisterContainerInstanceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeregisterContainerInstanceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterContainerInstanceError {}
/// Errors returned by DeregisterTaskDefinition
#[derive(Debug, PartialEq)]
pub enum DeregisterTaskDefinitionError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeregisterTaskDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterTaskDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeregisterTaskDefinitionError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeregisterTaskDefinitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DeregisterTaskDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterTaskDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterTaskDefinitionError::Client(ref cause) => write!(f, "{}", cause),
            DeregisterTaskDefinitionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeregisterTaskDefinitionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterTaskDefinitionError {}
/// Errors returned by DescribeCapacityProviders
#[derive(Debug, PartialEq)]
pub enum DescribeCapacityProvidersError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeCapacityProvidersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCapacityProvidersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeCapacityProvidersError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeCapacityProvidersError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeCapacityProvidersError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCapacityProvidersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCapacityProvidersError::Client(ref cause) => write!(f, "{}", cause),
            DescribeCapacityProvidersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeCapacityProvidersError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCapacityProvidersError {}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeClustersError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeClustersError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeClustersError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeClustersError::Client(ref cause) => write!(f, "{}", cause),
            DescribeClustersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeClustersError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeClustersError {}
/// Errors returned by DescribeContainerInstances
#[derive(Debug, PartialEq)]
pub enum DescribeContainerInstancesError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeContainerInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeContainerInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeContainerInstancesError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DescribeContainerInstancesError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeContainerInstancesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeContainerInstancesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeContainerInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeContainerInstancesError::Client(ref cause) => write!(f, "{}", cause),
            DescribeContainerInstancesError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DescribeContainerInstancesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeContainerInstancesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeContainerInstancesError {}
/// Errors returned by DescribeServices
#[derive(Debug, PartialEq)]
pub enum DescribeServicesError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeServicesError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DescribeServicesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeServicesError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeServicesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeServicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServicesError::Client(ref cause) => write!(f, "{}", cause),
            DescribeServicesError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DescribeServicesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeServicesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServicesError {}
/// Errors returned by DescribeTaskDefinition
#[derive(Debug, PartialEq)]
pub enum DescribeTaskDefinitionError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeTaskDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTaskDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeTaskDefinitionError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeTaskDefinitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeTaskDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTaskDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTaskDefinitionError::Client(ref cause) => write!(f, "{}", cause),
            DescribeTaskDefinitionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeTaskDefinitionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTaskDefinitionError {}
/// Errors returned by DescribeTaskSets
#[derive(Debug, PartialEq)]
pub enum DescribeTaskSetsError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service isn't active. You can't update a service that's inactive. If you have previously deleted a service, you can re-create it with <a>CreateService</a>.</p>
    ServiceNotActive(String),
    /// <p>The specified service wasn't found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster specific and Region specific.</p>
    ServiceNotFound(String),
    /// <p>The specified task isn't supported in this Region.</p>
    UnsupportedFeature(String),
}

impl DescribeTaskSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTaskSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeTaskSetsError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(DescribeTaskSetsError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DescribeTaskSetsError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeTaskSetsError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeTaskSetsError::Server(err.msg))
                }
                "ServiceNotActiveException" => {
                    return RusotoError::Service(DescribeTaskSetsError::ServiceNotActive(err.msg))
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(DescribeTaskSetsError::ServiceNotFound(err.msg))
                }
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(DescribeTaskSetsError::UnsupportedFeature(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTaskSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTaskSetsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeTaskSetsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeTaskSetsError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTaskSetsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeTaskSetsError::Server(ref cause) => write!(f, "{}", cause),
            DescribeTaskSetsError::ServiceNotActive(ref cause) => write!(f, "{}", cause),
            DescribeTaskSetsError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTaskSetsError::UnsupportedFeature(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTaskSetsError {}
/// Errors returned by DescribeTasks
#[derive(Debug, PartialEq)]
pub enum DescribeTasksError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeTasksError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DescribeTasksError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeTasksError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeTasksError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTasksError::Client(ref cause) => write!(f, "{}", cause),
            DescribeTasksError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTasksError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeTasksError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTasksError {}
/// Errors returned by DiscoverPollEndpoint
#[derive(Debug, PartialEq)]
pub enum DiscoverPollEndpointError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DiscoverPollEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DiscoverPollEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DiscoverPollEndpointError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DiscoverPollEndpointError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DiscoverPollEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DiscoverPollEndpointError::Client(ref cause) => write!(f, "{}", cause),
            DiscoverPollEndpointError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DiscoverPollEndpointError {}
/// Errors returned by ExecuteCommand
#[derive(Debug, PartialEq)]
pub enum ExecuteCommandError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The execute command cannot run. This error can be caused by any of the following configuration issues:</p> <ul> <li> <p>Incorrect IAM permissions</p> </li> <li> <p>The SSM agent is not installed or is not running</p> </li> <li> <p> There is an interface Amazon VPC endpoint for Amazon ECS, but there is not one for for Systems Manager Session Manager</p> </li> </ul> <p>For information about how to troubleshoot the issues, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-exec.html">Troubleshooting issues with ECS Exec</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    TargetNotConnected(String),
}

impl ExecuteCommandError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExecuteCommandError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ExecuteCommandError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(ExecuteCommandError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ExecuteCommandError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ExecuteCommandError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ExecuteCommandError::Server(err.msg))
                }
                "TargetNotConnectedException" => {
                    return RusotoError::Service(ExecuteCommandError::TargetNotConnected(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExecuteCommandError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteCommandError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ExecuteCommandError::Client(ref cause) => write!(f, "{}", cause),
            ExecuteCommandError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            ExecuteCommandError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ExecuteCommandError::Server(ref cause) => write!(f, "{}", cause),
            ExecuteCommandError::TargetNotConnected(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExecuteCommandError {}
/// Errors returned by ListAccountSettings
#[derive(Debug, PartialEq)]
pub enum ListAccountSettingsError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListAccountSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListAccountSettingsError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAccountSettingsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListAccountSettingsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAccountSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAccountSettingsError::Client(ref cause) => write!(f, "{}", cause),
            ListAccountSettingsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListAccountSettingsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAccountSettingsError {}
/// Errors returned by ListAttributes
#[derive(Debug, PartialEq)]
pub enum ListAttributesError {
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
}

impl ListAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListAttributesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAttributesError::InvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAttributesError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            ListAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAttributesError {}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListClustersError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListClustersError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ListClustersError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListClustersError::Client(ref cause) => write!(f, "{}", cause),
            ListClustersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListClustersError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListClustersError {}
/// Errors returned by ListContainerInstances
#[derive(Debug, PartialEq)]
pub enum ListContainerInstancesError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListContainerInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListContainerInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListContainerInstancesError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListContainerInstancesError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListContainerInstancesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListContainerInstancesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListContainerInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListContainerInstancesError::Client(ref cause) => write!(f, "{}", cause),
            ListContainerInstancesError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            ListContainerInstancesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListContainerInstancesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListContainerInstancesError {}
/// Errors returned by ListServices
#[derive(Debug, PartialEq)]
pub enum ListServicesError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListServicesError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListServicesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListServicesError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ListServicesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListServicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServicesError::Client(ref cause) => write!(f, "{}", cause),
            ListServicesError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            ListServicesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListServicesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListServicesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTagsForResourceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::Client(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTaskDefinitionFamilies
#[derive(Debug, PartialEq)]
pub enum ListTaskDefinitionFamiliesError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListTaskDefinitionFamiliesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTaskDefinitionFamiliesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTaskDefinitionFamiliesError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTaskDefinitionFamiliesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListTaskDefinitionFamiliesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTaskDefinitionFamiliesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTaskDefinitionFamiliesError::Client(ref cause) => write!(f, "{}", cause),
            ListTaskDefinitionFamiliesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTaskDefinitionFamiliesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTaskDefinitionFamiliesError {}
/// Errors returned by ListTaskDefinitions
#[derive(Debug, PartialEq)]
pub enum ListTaskDefinitionsError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListTaskDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTaskDefinitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTaskDefinitionsError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTaskDefinitionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListTaskDefinitionsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTaskDefinitionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTaskDefinitionsError::Client(ref cause) => write!(f, "{}", cause),
            ListTaskDefinitionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTaskDefinitionsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTaskDefinitionsError {}
/// Errors returned by ListTasks
#[derive(Debug, PartialEq)]
pub enum ListTasksError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service wasn't found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster specific and Region specific.</p>
    ServiceNotFound(String),
}

impl ListTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(ListTasksError::Client(err.msg)),
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListTasksError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTasksError::InvalidParameter(err.msg))
                }
                "ServerException" => return RusotoError::Service(ListTasksError::Server(err.msg)),
                "ServiceNotFoundException" => {
                    return RusotoError::Service(ListTasksError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTasksError::Client(ref cause) => write!(f, "{}", cause),
            ListTasksError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            ListTasksError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTasksError::Server(ref cause) => write!(f, "{}", cause),
            ListTasksError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTasksError {}
/// Errors returned by PutAccountSetting
#[derive(Debug, PartialEq)]
pub enum PutAccountSettingError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl PutAccountSettingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAccountSettingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(PutAccountSettingError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAccountSettingError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(PutAccountSettingError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAccountSettingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccountSettingError::Client(ref cause) => write!(f, "{}", cause),
            PutAccountSettingError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutAccountSettingError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAccountSettingError {}
/// Errors returned by PutAccountSettingDefault
#[derive(Debug, PartialEq)]
pub enum PutAccountSettingDefaultError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl PutAccountSettingDefaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAccountSettingDefaultError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(PutAccountSettingDefaultError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAccountSettingDefaultError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(PutAccountSettingDefaultError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAccountSettingDefaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccountSettingDefaultError::Client(ref cause) => write!(f, "{}", cause),
            PutAccountSettingDefaultError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutAccountSettingDefaultError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAccountSettingDefaultError {}
/// Errors returned by PutAttributes
#[derive(Debug, PartialEq)]
pub enum PutAttributesError {
    /// <p>You can apply up to 10 custom attributes for each resource. You can view the attributes of a resource with <a>ListAttributes</a>. You can remove existing attributes on a resource with <a>DeleteAttributes</a>.</p>
    AttributeLimitExceeded(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified target wasn't found. You can view your available container instances with <a>ListContainerInstances</a>. Amazon ECS container instances are cluster-specific and Region-specific.</p>
    TargetNotFound(String),
}

impl PutAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AttributeLimitExceededException" => {
                    return RusotoError::Service(PutAttributesError::AttributeLimitExceeded(
                        err.msg,
                    ))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(PutAttributesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAttributesError::InvalidParameter(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(PutAttributesError::TargetNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAttributesError::AttributeLimitExceeded(ref cause) => write!(f, "{}", cause),
            PutAttributesError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            PutAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutAttributesError::TargetNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAttributesError {}
/// Errors returned by PutClusterCapacityProviders
#[derive(Debug, PartialEq)]
pub enum PutClusterCapacityProvidersError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource is in-use and can't be removed.</p>
    ResourceInUse(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>There's already a current Amazon ECS container agent update in progress on the container instance that's specified. If the container agent becomes disconnected while it's in a transitional stage, such as <code>PENDING</code> or <code>STAGING</code>, the update process can get stuck in that state. However, when the agent reconnects, it resumes where it stopped previously.</p>
    UpdateInProgress(String),
}

impl PutClusterCapacityProvidersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutClusterCapacityProvidersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(PutClusterCapacityProvidersError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(PutClusterCapacityProvidersError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        PutClusterCapacityProvidersError::InvalidParameter(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(PutClusterCapacityProvidersError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(PutClusterCapacityProvidersError::Server(err.msg))
                }
                "UpdateInProgressException" => {
                    return RusotoError::Service(
                        PutClusterCapacityProvidersError::UpdateInProgress(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutClusterCapacityProvidersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutClusterCapacityProvidersError::Client(ref cause) => write!(f, "{}", cause),
            PutClusterCapacityProvidersError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            PutClusterCapacityProvidersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutClusterCapacityProvidersError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            PutClusterCapacityProvidersError::Server(ref cause) => write!(f, "{}", cause),
            PutClusterCapacityProvidersError::UpdateInProgress(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutClusterCapacityProvidersError {}
/// Errors returned by RegisterContainerInstance
#[derive(Debug, PartialEq)]
pub enum RegisterContainerInstanceError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl RegisterContainerInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterContainerInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RegisterContainerInstanceError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterContainerInstanceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(RegisterContainerInstanceError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterContainerInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterContainerInstanceError::Client(ref cause) => write!(f, "{}", cause),
            RegisterContainerInstanceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RegisterContainerInstanceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterContainerInstanceError {}
/// Errors returned by RegisterTaskDefinition
#[derive(Debug, PartialEq)]
pub enum RegisterTaskDefinitionError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl RegisterTaskDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterTaskDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RegisterTaskDefinitionError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterTaskDefinitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(RegisterTaskDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterTaskDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterTaskDefinitionError::Client(ref cause) => write!(f, "{}", cause),
            RegisterTaskDefinitionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RegisterTaskDefinitionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterTaskDefinitionError {}
/// Errors returned by RunTask
#[derive(Debug, PartialEq)]
pub enum RunTaskError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>Your Amazon Web Services account was blocked. For more information, contact <a href="http://aws.amazon.com/contact-us/"> Amazon Web Services Support</a>.</p>
    Blocked(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified platform version doesn't satisfy the required capabilities of the task definition.</p>
    PlatformTaskDefinitionIncompatibility(String),
    /// <p>The specified platform version doesn't exist.</p>
    PlatformUnknown(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified task isn't supported in this Region.</p>
    UnsupportedFeature(String),
}

impl RunTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RunTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RunTaskError::AccessDenied(err.msg))
                }
                "BlockedException" => return RusotoError::Service(RunTaskError::Blocked(err.msg)),
                "ClientException" => return RusotoError::Service(RunTaskError::Client(err.msg)),
                "ClusterNotFoundException" => {
                    return RusotoError::Service(RunTaskError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RunTaskError::InvalidParameter(err.msg))
                }
                "PlatformTaskDefinitionIncompatibilityException" => {
                    return RusotoError::Service(
                        RunTaskError::PlatformTaskDefinitionIncompatibility(err.msg),
                    )
                }
                "PlatformUnknownException" => {
                    return RusotoError::Service(RunTaskError::PlatformUnknown(err.msg))
                }
                "ServerException" => return RusotoError::Service(RunTaskError::Server(err.msg)),
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(RunTaskError::UnsupportedFeature(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RunTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RunTaskError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RunTaskError::Blocked(ref cause) => write!(f, "{}", cause),
            RunTaskError::Client(ref cause) => write!(f, "{}", cause),
            RunTaskError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            RunTaskError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RunTaskError::PlatformTaskDefinitionIncompatibility(ref cause) => {
                write!(f, "{}", cause)
            }
            RunTaskError::PlatformUnknown(ref cause) => write!(f, "{}", cause),
            RunTaskError::Server(ref cause) => write!(f, "{}", cause),
            RunTaskError::UnsupportedFeature(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RunTaskError {}
/// Errors returned by StartTask
#[derive(Debug, PartialEq)]
pub enum StartTaskError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl StartTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(StartTaskError::Client(err.msg)),
                "ClusterNotFoundException" => {
                    return RusotoError::Service(StartTaskError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartTaskError::InvalidParameter(err.msg))
                }
                "ServerException" => return RusotoError::Service(StartTaskError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartTaskError::Client(ref cause) => write!(f, "{}", cause),
            StartTaskError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            StartTaskError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartTaskError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartTaskError {}
/// Errors returned by StopTask
#[derive(Debug, PartialEq)]
pub enum StopTaskError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl StopTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(StopTaskError::Client(err.msg)),
                "ClusterNotFoundException" => {
                    return RusotoError::Service(StopTaskError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopTaskError::InvalidParameter(err.msg))
                }
                "ServerException" => return RusotoError::Service(StopTaskError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopTaskError::Client(ref cause) => write!(f, "{}", cause),
            StopTaskError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            StopTaskError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopTaskError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopTaskError {}
/// Errors returned by SubmitAttachmentStateChanges
#[derive(Debug, PartialEq)]
pub enum SubmitAttachmentStateChangesError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl SubmitAttachmentStateChangesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SubmitAttachmentStateChangesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SubmitAttachmentStateChangesError::AccessDenied(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(SubmitAttachmentStateChangesError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        SubmitAttachmentStateChangesError::InvalidParameter(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(SubmitAttachmentStateChangesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubmitAttachmentStateChangesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubmitAttachmentStateChangesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SubmitAttachmentStateChangesError::Client(ref cause) => write!(f, "{}", cause),
            SubmitAttachmentStateChangesError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            SubmitAttachmentStateChangesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubmitAttachmentStateChangesError {}
/// Errors returned by SubmitContainerStateChange
#[derive(Debug, PartialEq)]
pub enum SubmitContainerStateChangeError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl SubmitContainerStateChangeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SubmitContainerStateChangeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SubmitContainerStateChangeError::AccessDenied(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(SubmitContainerStateChangeError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(SubmitContainerStateChangeError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubmitContainerStateChangeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubmitContainerStateChangeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SubmitContainerStateChangeError::Client(ref cause) => write!(f, "{}", cause),
            SubmitContainerStateChangeError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubmitContainerStateChangeError {}
/// Errors returned by SubmitTaskStateChange
#[derive(Debug, PartialEq)]
pub enum SubmitTaskStateChangeError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl SubmitTaskStateChangeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubmitTaskStateChangeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SubmitTaskStateChangeError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(SubmitTaskStateChangeError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SubmitTaskStateChangeError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(SubmitTaskStateChangeError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubmitTaskStateChangeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubmitTaskStateChangeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SubmitTaskStateChangeError::Client(ref cause) => write!(f, "{}", cause),
            SubmitTaskStateChangeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SubmitTaskStateChangeError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubmitTaskStateChangeError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource wasn't found.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(TagResourceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(TagResourceError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::Client(ref cause) => write!(f, "{}", cause),
            TagResourceError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource wasn't found.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UntagResourceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UntagResourceError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::Client(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateCapacityProvider
#[derive(Debug, PartialEq)]
pub enum UpdateCapacityProviderError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateCapacityProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCapacityProviderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateCapacityProviderError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateCapacityProviderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateCapacityProviderError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCapacityProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCapacityProviderError::Client(ref cause) => write!(f, "{}", cause),
            UpdateCapacityProviderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateCapacityProviderError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateCapacityProviderError {}
/// Errors returned by UpdateCluster
#[derive(Debug, PartialEq)]
pub enum UpdateClusterError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateClusterError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UpdateClusterError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateClusterError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateClusterError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateClusterError::Client(ref cause) => write!(f, "{}", cause),
            UpdateClusterError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            UpdateClusterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateClusterError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateClusterError {}
/// Errors returned by UpdateClusterSettings
#[derive(Debug, PartialEq)]
pub enum UpdateClusterSettingsError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateClusterSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClusterSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateClusterSettingsError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UpdateClusterSettingsError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateClusterSettingsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateClusterSettingsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateClusterSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateClusterSettingsError::Client(ref cause) => write!(f, "{}", cause),
            UpdateClusterSettingsError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            UpdateClusterSettingsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateClusterSettingsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateClusterSettingsError {}
/// Errors returned by UpdateContainerAgent
#[derive(Debug, PartialEq)]
pub enum UpdateContainerAgentError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>Amazon ECS can't determine the current version of the Amazon ECS container agent on the container instance and doesn't have enough information to proceed with an update. This could be because the agent running on the container instance is a previous or custom version that doesn't use our version information.</p>
    MissingVersion(String),
    /// <p>There's no update available for this Amazon ECS container agent. This might be because the agent is already running the latest version or because it's so old that there's no update path to the current version.</p>
    NoUpdateAvailable(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>There's already a current Amazon ECS container agent update in progress on the container instance that's specified. If the container agent becomes disconnected while it's in a transitional stage, such as <code>PENDING</code> or <code>STAGING</code>, the update process can get stuck in that state. However, when the agent reconnects, it resumes where it stopped previously.</p>
    UpdateInProgress(String),
}

impl UpdateContainerAgentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContainerAgentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateContainerAgentError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UpdateContainerAgentError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateContainerAgentError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingVersionException" => {
                    return RusotoError::Service(UpdateContainerAgentError::MissingVersion(err.msg))
                }
                "NoUpdateAvailableException" => {
                    return RusotoError::Service(UpdateContainerAgentError::NoUpdateAvailable(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateContainerAgentError::Server(err.msg))
                }
                "UpdateInProgressException" => {
                    return RusotoError::Service(UpdateContainerAgentError::UpdateInProgress(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateContainerAgentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContainerAgentError::Client(ref cause) => write!(f, "{}", cause),
            UpdateContainerAgentError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            UpdateContainerAgentError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateContainerAgentError::MissingVersion(ref cause) => write!(f, "{}", cause),
            UpdateContainerAgentError::NoUpdateAvailable(ref cause) => write!(f, "{}", cause),
            UpdateContainerAgentError::Server(ref cause) => write!(f, "{}", cause),
            UpdateContainerAgentError::UpdateInProgress(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContainerAgentError {}
/// Errors returned by UpdateContainerInstancesState
#[derive(Debug, PartialEq)]
pub enum UpdateContainerInstancesStateError {
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateContainerInstancesStateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateContainerInstancesStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateContainerInstancesStateError::Client(
                        err.msg,
                    ))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(
                        UpdateContainerInstancesStateError::ClusterNotFound(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateContainerInstancesStateError::InvalidParameter(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateContainerInstancesStateError::Server(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateContainerInstancesStateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContainerInstancesStateError::Client(ref cause) => write!(f, "{}", cause),
            UpdateContainerInstancesStateError::ClusterNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateContainerInstancesStateError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateContainerInstancesStateError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContainerInstancesStateError {}
/// Errors returned by UpdateService
#[derive(Debug, PartialEq)]
pub enum UpdateServiceError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified platform version doesn't satisfy the required capabilities of the task definition.</p>
    PlatformTaskDefinitionIncompatibility(String),
    /// <p>The specified platform version doesn't exist.</p>
    PlatformUnknown(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service isn't active. You can't update a service that's inactive. If you have previously deleted a service, you can re-create it with <a>CreateService</a>.</p>
    ServiceNotActive(String),
    /// <p>The specified service wasn't found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster specific and Region specific.</p>
    ServiceNotFound(String),
}

impl UpdateServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateServiceError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(UpdateServiceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UpdateServiceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateServiceError::InvalidParameter(err.msg))
                }
                "PlatformTaskDefinitionIncompatibilityException" => {
                    return RusotoError::Service(
                        UpdateServiceError::PlatformTaskDefinitionIncompatibility(err.msg),
                    )
                }
                "PlatformUnknownException" => {
                    return RusotoError::Service(UpdateServiceError::PlatformUnknown(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateServiceError::Server(err.msg))
                }
                "ServiceNotActiveException" => {
                    return RusotoError::Service(UpdateServiceError::ServiceNotActive(err.msg))
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(UpdateServiceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServiceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::Client(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::PlatformTaskDefinitionIncompatibility(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateServiceError::PlatformUnknown(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::Server(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::ServiceNotActive(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateServiceError {}
/// Errors returned by UpdateServicePrimaryTaskSet
#[derive(Debug, PartialEq)]
pub enum UpdateServicePrimaryTaskSetError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service isn't active. You can't update a service that's inactive. If you have previously deleted a service, you can re-create it with <a>CreateService</a>.</p>
    ServiceNotActive(String),
    /// <p>The specified service wasn't found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster specific and Region specific.</p>
    ServiceNotFound(String),
    /// <p>The specified task set wasn't found. You can view your available task sets with <a>DescribeTaskSets</a>. Task sets are specific to each cluster, service and Region.</p>
    TaskSetNotFound(String),
    /// <p>The specified task isn't supported in this Region.</p>
    UnsupportedFeature(String),
}

impl UpdateServicePrimaryTaskSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateServicePrimaryTaskSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateServicePrimaryTaskSetError::AccessDenied(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(UpdateServicePrimaryTaskSetError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UpdateServicePrimaryTaskSetError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateServicePrimaryTaskSetError::InvalidParameter(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateServicePrimaryTaskSetError::Server(err.msg))
                }
                "ServiceNotActiveException" => {
                    return RusotoError::Service(
                        UpdateServicePrimaryTaskSetError::ServiceNotActive(err.msg),
                    )
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(UpdateServicePrimaryTaskSetError::ServiceNotFound(
                        err.msg,
                    ))
                }
                "TaskSetNotFoundException" => {
                    return RusotoError::Service(UpdateServicePrimaryTaskSetError::TaskSetNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(
                        UpdateServicePrimaryTaskSetError::UnsupportedFeature(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateServicePrimaryTaskSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServicePrimaryTaskSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateServicePrimaryTaskSetError::Client(ref cause) => write!(f, "{}", cause),
            UpdateServicePrimaryTaskSetError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            UpdateServicePrimaryTaskSetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateServicePrimaryTaskSetError::Server(ref cause) => write!(f, "{}", cause),
            UpdateServicePrimaryTaskSetError::ServiceNotActive(ref cause) => write!(f, "{}", cause),
            UpdateServicePrimaryTaskSetError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateServicePrimaryTaskSetError::TaskSetNotFound(ref cause) => write!(f, "{}", cause),
            UpdateServicePrimaryTaskSetError::UnsupportedFeature(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateServicePrimaryTaskSetError {}
/// Errors returned by UpdateTaskSet
#[derive(Debug, PartialEq)]
pub enum UpdateTaskSetError {
    /// <p>You don't have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permissions to use the action or resource,. Or, it might be specifying an identifier that isn't valid.</p>
    Client(String),
    /// <p>The specified cluster wasn't found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter isn't valid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service isn't active. You can't update a service that's inactive. If you have previously deleted a service, you can re-create it with <a>CreateService</a>.</p>
    ServiceNotActive(String),
    /// <p>The specified service wasn't found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster specific and Region specific.</p>
    ServiceNotFound(String),
    /// <p>The specified task set wasn't found. You can view your available task sets with <a>DescribeTaskSets</a>. Task sets are specific to each cluster, service and Region.</p>
    TaskSetNotFound(String),
    /// <p>The specified task isn't supported in this Region.</p>
    UnsupportedFeature(String),
}

impl UpdateTaskSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTaskSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateTaskSetError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(UpdateTaskSetError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UpdateTaskSetError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateTaskSetError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateTaskSetError::Server(err.msg))
                }
                "ServiceNotActiveException" => {
                    return RusotoError::Service(UpdateTaskSetError::ServiceNotActive(err.msg))
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(UpdateTaskSetError::ServiceNotFound(err.msg))
                }
                "TaskSetNotFoundException" => {
                    return RusotoError::Service(UpdateTaskSetError::TaskSetNotFound(err.msg))
                }
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(UpdateTaskSetError::UnsupportedFeature(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTaskSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTaskSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateTaskSetError::Client(ref cause) => write!(f, "{}", cause),
            UpdateTaskSetError::ClusterNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTaskSetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateTaskSetError::Server(ref cause) => write!(f, "{}", cause),
            UpdateTaskSetError::ServiceNotActive(ref cause) => write!(f, "{}", cause),
            UpdateTaskSetError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTaskSetError::TaskSetNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTaskSetError::UnsupportedFeature(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTaskSetError {}
/// Trait representing the capabilities of the Amazon ECS API. Amazon ECS clients implement this trait.
#[async_trait]
pub trait Ecs {
    /// <p>Creates a new capacity provider. Capacity providers are associated with an Amazon ECS cluster and are used in capacity provider strategies to facilitate cluster auto scaling.</p> <p>Only capacity providers that use an Auto Scaling group can be created. Amazon ECS tasks on Fargate use the <code>FARGATE</code> and <code>FARGATE_SPOT</code> capacity providers. These providers are available to all accounts in the Amazon Web Services Regions that Fargate supports.</p>
    async fn create_capacity_provider(
        &self,
        input: CreateCapacityProviderRequest,
    ) -> Result<CreateCapacityProviderResponse, RusotoError<CreateCapacityProviderError>>;

    /// <p><p>Creates a new Amazon ECS cluster. By default, your account receives a <code>default</code> cluster when you launch your first container instance. However, you can create your own cluster with a unique name with the <code>CreateCluster</code> action.</p> <note> <p>When you call the <a>CreateCluster</a> API operation, Amazon ECS attempts to create the Amazon ECS service-linked role for your account. This is so that it can manage required resources in other Amazon Web Services services on your behalf. However, if the IAM user that makes the call doesn&#39;t have permissions to create the service-linked role, it isn&#39;t created. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using service-linked roles for Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>>;

    /// <p><p>Runs and maintains your desired number of tasks from a specified task definition. If the number of tasks running in a service drops below the <code>desiredCount</code>, Amazon ECS runs another copy of the task in the specified cluster. To update an existing service, see the <a>UpdateService</a> action.</p> <p>In addition to maintaining the desired count of tasks in your service, you can optionally run your service behind one or more load balancers. The load balancers distribute traffic across the tasks that are associated with the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-load-balancing.html">Service load balancing</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>Tasks for services that don&#39;t use a load balancer are considered healthy if they&#39;re in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they&#39;re in the <code>RUNNING</code> state and are reported as healthy by the load balancer.</p> <p>There are two service scheduler strategies available:</p> <ul> <li> <p> <code>REPLICA</code> - The replica scheduling strategy places and maintains your desired number of tasks across your cluster. By default, the service scheduler spreads tasks across Availability Zones. You can use task placement strategies and constraints to customize task placement decisions. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Service scheduler concepts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li> <li> <p> <code>DAEMON</code> - The daemon scheduling strategy deploys exactly one task on each active container instance that meets all of the task placement constraints that you specify in your cluster. The service scheduler also evaluates the task placement constraints for running tasks. It also stops tasks that don&#39;t meet the placement constraints. When using this strategy, you don&#39;t need to specify a desired number of tasks, a task placement strategy, or use Service Auto Scaling policies. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Service scheduler concepts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li> </ul> <p>You can optionally specify a deployment configuration for your service. The deployment is initiated by changing properties. For example, the deployment might be initiated by the task definition or by your desired count of a service. This is done with an <a>UpdateService</a> operation. The default value for a replica service for <code>minimumHealthyPercent</code> is 100%. The default value for a daemon service for <code>minimumHealthyPercent</code> is 0%.</p> <p>If a service uses the <code>ECS</code> deployment controller, the minimum healthy percent represents a lower limit on the number of tasks in a service that must remain in the <code>RUNNING</code> state during a deployment. Specifically, it represents it as a percentage of your desired number of tasks (rounded up to the nearest integer). This happens when any of your container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. Using this parameter, you can deploy without using additional cluster capacity. For example, if you set your service to have desired number of four tasks and a minimum healthy percent of 50%, the scheduler might stop two existing tasks to free up cluster capacity before starting two new tasks. If they&#39;re in the <code>RUNNING</code> state, tasks for services that don&#39;t use a load balancer are considered healthy . If they&#39;re in the <code>RUNNING</code> state and reported as healthy by the load balancer, tasks for services that <i>do</i> use a load balancer are considered healthy . The default value for minimum healthy percent is 100%.</p> <p>If a service uses the <code>ECS</code> deployment controller, the <b>maximum percent</b> parameter represents an upper limit on the number of tasks in a service that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment. Specifically, it represents it as a percentage of the desired number of tasks (rounded down to the nearest integer). This happens when any of your container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. Using this parameter, you can define the deployment batch size. For example, if your service has a desired number of four tasks and a maximum percent value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default value for maximum percent is 200%.</p> <p>If a service uses either the <code>CODE_DEPLOY</code> or <code>EXTERNAL</code> deployment controller types and tasks that use the EC2 launch type, the <b>minimum healthy percent</b> and <b>maximum percent</b> values are used only to define the lower and upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state. This is while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the minimum healthy percent and maximum percent values aren&#39;t used. This is the case even if they&#39;re currently visible when describing your service.</p> <p>When creating a service that uses the <code>EXTERNAL</code> deployment controller, you can specify only parameters that aren&#39;t controlled at the task set level. The only required parameter is the service name. You control your services using the <a>CreateTaskSet</a> operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster using the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support the task definition of your service. For example, they have the required CPU, memory, ports, and container instance attributes.</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner. This is the case even if you can choose a different placement strategy with the <code>placementStrategy</code> parameter.</p> <ul> <li> <p>Sort the valid container instances, giving priority to instances that have the fewest number of running tasks for this service in their respective Availability Zone. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone based on the previous steps, favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul></p>
    async fn create_service(
        &self,
        input: CreateServiceRequest,
    ) -> Result<CreateServiceResponse, RusotoError<CreateServiceError>>;

    /// <p>Create a task set in the specified cluster and service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn create_task_set(
        &self,
        input: CreateTaskSetRequest,
    ) -> Result<CreateTaskSetResponse, RusotoError<CreateTaskSetError>>;

    /// <p>Disables an account setting for a specified IAM user, IAM role, or the root user for an account.</p>
    async fn delete_account_setting(
        &self,
        input: DeleteAccountSettingRequest,
    ) -> Result<DeleteAccountSettingResponse, RusotoError<DeleteAccountSettingError>>;

    /// <p>Deletes one or more custom attributes from an Amazon ECS resource.</p>
    async fn delete_attributes(
        &self,
        input: DeleteAttributesRequest,
    ) -> Result<DeleteAttributesResponse, RusotoError<DeleteAttributesError>>;

    /// <p>Deletes the specified capacity provider.</p> <note> <p>The <code>FARGATE</code> and <code>FARGATE_SPOT</code> capacity providers are reserved and can't be deleted. You can disassociate them from a cluster using either the <a>PutClusterCapacityProviders</a> API or by deleting the cluster.</p> </note> <p>Prior to a capacity provider being deleted, the capacity provider must be removed from the capacity provider strategy from all services. The <a>UpdateService</a> API can be used to remove a capacity provider from a service's capacity provider strategy. When updating a service, the <code>forceNewDeployment</code> option can be used to ensure that any tasks using the Amazon EC2 instance capacity provided by the capacity provider are transitioned to use the capacity from the remaining capacity providers. Only capacity providers that aren't associated with a cluster can be deleted. To remove a capacity provider from a cluster, you can either use <a>PutClusterCapacityProviders</a> or delete the cluster.</p>
    async fn delete_capacity_provider(
        &self,
        input: DeleteCapacityProviderRequest,
    ) -> Result<DeleteCapacityProviderResponse, RusotoError<DeleteCapacityProviderError>>;

    /// <p>Deletes the specified cluster. The cluster transitions to the <code>INACTIVE</code> state. Clusters with an <code>INACTIVE</code> status might remain discoverable in your account for a period of time. However, this behavior is subject to change in the future. We don't recommend that you rely on <code>INACTIVE</code> clusters persisting.</p> <p>You must deregister all container instances from this cluster before you may delete it. You can list the container instances in a cluster with <a>ListContainerInstances</a> and deregister them with <a>DeregisterContainerInstance</a>.</p>
    async fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, RusotoError<DeleteClusterError>>;

    /// <p><p>Deletes a specified service within a cluster. You can delete a service if you have no running tasks in it and the desired task count is zero. If the service is actively maintaining tasks, you can&#39;t delete it, and you must update the service to a desired task count of zero. For more information, see <a>UpdateService</a>.</p> <note> <p>When you delete a service, if there are still running tasks that require cleanup, the service status moves from <code>ACTIVE</code> to <code>DRAINING</code>, and the service is no longer visible in the console or in the <a>ListServices</a> API operation. After all tasks have transitioned to either <code>STOPPING</code> or <code>STOPPED</code> status, the service status moves from <code>DRAINING</code> to <code>INACTIVE</code>. Services in the <code>DRAINING</code> or <code>INACTIVE</code> status can still be viewed with the <a>DescribeServices</a> API operation. However, in the future, <code>INACTIVE</code> services may be cleaned up and purged from Amazon ECS record keeping, and <a>DescribeServices</a> calls on those services return a <code>ServiceNotFoundException</code> error.</p> </note> <important> <p>If you attempt to create a new service with the same name as an existing service in either <code>ACTIVE</code> or <code>DRAINING</code> status, you receive an error.</p> </important></p>
    async fn delete_service(
        &self,
        input: DeleteServiceRequest,
    ) -> Result<DeleteServiceResponse, RusotoError<DeleteServiceError>>;

    /// <p>Deletes a specified task set within a service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn delete_task_set(
        &self,
        input: DeleteTaskSetRequest,
    ) -> Result<DeleteTaskSetResponse, RusotoError<DeleteTaskSetError>>;

    /// <p><p>Deregisters an Amazon ECS container instance from the specified cluster. This instance is no longer available to run tasks.</p> <p>If you intend to use the container instance for some other purpose after deregistration, we recommend that you stop all of the tasks running on the container instance before deregistration. That prevents any orphaned tasks from consuming resources.</p> <p>Deregistering a container instance removes the instance from a cluster, but it doesn&#39;t terminate the EC2 instance. If you are finished using the instance, be sure to terminate it in the Amazon EC2 console to stop billing.</p> <note> <p>If you terminate a running container instance, Amazon ECS automatically deregisters the instance from your cluster (stopped container instances or instances with disconnected agents aren&#39;t automatically deregistered when terminated).</p> </note></p>
    async fn deregister_container_instance(
        &self,
        input: DeregisterContainerInstanceRequest,
    ) -> Result<DeregisterContainerInstanceResponse, RusotoError<DeregisterContainerInstanceError>>;

    /// <p><p>Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as <code>INACTIVE</code>. Existing tasks and services that reference an <code>INACTIVE</code> task definition continue to run without disruption. Existing services that reference an <code>INACTIVE</code> task definition can still scale up or down by modifying the service&#39;s desired count.</p> <p>You can&#39;t use an <code>INACTIVE</code> task definition to run new tasks or create new services, and you can&#39;t update an existing service to reference an <code>INACTIVE</code> task definition. However, there may be up to a 10-minute window following deregistration where these restrictions have not yet taken effect.</p> <note> <p>At this time, <code>INACTIVE</code> task definitions remain discoverable in your account indefinitely. However, this behavior is subject to change in the future. We don&#39;t recommend that you rely on <code>INACTIVE</code> task definitions persisting beyond the lifecycle of any associated tasks and services.</p> </note></p>
    async fn deregister_task_definition(
        &self,
        input: DeregisterTaskDefinitionRequest,
    ) -> Result<DeregisterTaskDefinitionResponse, RusotoError<DeregisterTaskDefinitionError>>;

    /// <p>Describes one or more of your capacity providers.</p>
    async fn describe_capacity_providers(
        &self,
        input: DescribeCapacityProvidersRequest,
    ) -> Result<DescribeCapacityProvidersResponse, RusotoError<DescribeCapacityProvidersError>>;

    /// <p>Describes one or more of your clusters.</p>
    async fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse, RusotoError<DescribeClustersError>>;

    /// <p>Describes one or more container instances. Returns metadata about each container instance requested.</p>
    async fn describe_container_instances(
        &self,
        input: DescribeContainerInstancesRequest,
    ) -> Result<DescribeContainerInstancesResponse, RusotoError<DescribeContainerInstancesError>>;

    /// <p>Describes the specified services running in your cluster.</p>
    async fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse, RusotoError<DescribeServicesError>>;

    /// <p><p>Describes a task definition. You can specify a <code>family</code> and <code>revision</code> to find information about a specific task definition, or you can simply specify the family to find the latest <code>ACTIVE</code> revision in that family.</p> <note> <p>You can only describe <code>INACTIVE</code> task definitions while an active task or service references them.</p> </note></p>
    async fn describe_task_definition(
        &self,
        input: DescribeTaskDefinitionRequest,
    ) -> Result<DescribeTaskDefinitionResponse, RusotoError<DescribeTaskDefinitionError>>;

    /// <p>Describes the task sets in the specified cluster and service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn describe_task_sets(
        &self,
        input: DescribeTaskSetsRequest,
    ) -> Result<DescribeTaskSetsResponse, RusotoError<DescribeTaskSetsError>>;

    /// <p>Describes a specified task or tasks.</p> <p>Currently, stopped tasks appear in the returned results for at least one hour.</p>
    async fn describe_tasks(
        &self,
        input: DescribeTasksRequest,
    ) -> Result<DescribeTasksResponse, RusotoError<DescribeTasksError>>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Returns an endpoint for the Amazon ECS agent to poll for updates.</p></p>
    async fn discover_poll_endpoint(
        &self,
        input: DiscoverPollEndpointRequest,
    ) -> Result<DiscoverPollEndpointResponse, RusotoError<DiscoverPollEndpointError>>;

    /// <p>Runs a command remotely on a container within a task.</p> <p>If you use a condition key in your IAM policy to refine the conditions for the policy statement, for example limit the actions to a specific cluster, you recevie an <code>AccessDeniedException</code> when there is a mismatch between the condition key value and the corresponding parameter value.</p>
    async fn execute_command(
        &self,
        input: ExecuteCommandRequest,
    ) -> Result<ExecuteCommandResponse, RusotoError<ExecuteCommandError>>;

    /// <p>Lists the account settings for a specified principal.</p>
    async fn list_account_settings(
        &self,
        input: ListAccountSettingsRequest,
    ) -> Result<ListAccountSettingsResponse, RusotoError<ListAccountSettingsError>>;

    /// <p>Lists the attributes for Amazon ECS resources within a specified target type and cluster. When you specify a target type and cluster, <code>ListAttributes</code> returns a list of attribute objects, one for each attribute on each resource. You can filter the list of results to a single attribute name to only return results that have that name. You can also filter the results by attribute name and value. You can do this, for example, to see which container instances in a cluster are running a Linux AMI (<code>ecs.os-type=linux</code>). </p>
    async fn list_attributes(
        &self,
        input: ListAttributesRequest,
    ) -> Result<ListAttributesResponse, RusotoError<ListAttributesError>>;

    /// <p>Returns a list of existing clusters.</p>
    async fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> Result<ListClustersResponse, RusotoError<ListClustersError>>;

    /// <p>Returns a list of container instances in a specified cluster. You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements inside the <code>filter</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn list_container_instances(
        &self,
        input: ListContainerInstancesRequest,
    ) -> Result<ListContainerInstancesResponse, RusotoError<ListContainerInstancesError>>;

    /// <p>Returns a list of services. You can filter the results by cluster, launch type, and scheduling strategy.</p>
    async fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> Result<ListServicesResponse, RusotoError<ListServicesError>>;

    /// <p>List the tags for an Amazon ECS resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns a list of task definition families that are registered to your account. This list includes task definition families that no longer have any <code>ACTIVE</code> task definition revisions.</p> <p>You can filter out task definition families that don't contain any <code>ACTIVE</code> task definition revisions by setting the <code>status</code> parameter to <code>ACTIVE</code>. You can also filter the results with the <code>familyPrefix</code> parameter.</p>
    async fn list_task_definition_families(
        &self,
        input: ListTaskDefinitionFamiliesRequest,
    ) -> Result<ListTaskDefinitionFamiliesResponse, RusotoError<ListTaskDefinitionFamiliesError>>;

    /// <p>Returns a list of task definitions that are registered to your account. You can filter the results by family name with the <code>familyPrefix</code> parameter or by status with the <code>status</code> parameter.</p>
    async fn list_task_definitions(
        &self,
        input: ListTaskDefinitionsRequest,
    ) -> Result<ListTaskDefinitionsResponse, RusotoError<ListTaskDefinitionsError>>;

    /// <p>Returns a list of tasks. You can filter the results by cluster, task definition family, container instance, launch type, what IAM principal started the task, or by the desired status of the task.</p> <p>Recently stopped tasks might appear in the returned results. Currently, stopped tasks appear in the returned results for at least one hour.</p>
    async fn list_tasks(
        &self,
        input: ListTasksRequest,
    ) -> Result<ListTasksResponse, RusotoError<ListTasksError>>;

    /// <p>Modifies an account setting. Account settings are set on a per-Region basis.</p> <p>If you change the account setting for the root user, the default settings for all of the IAM users and roles that no individual account setting was specified are reset for. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html">Account Settings</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>When <code>serviceLongArnFormat</code>, <code>taskLongArnFormat</code>, or <code>containerInstanceLongArnFormat</code> are specified, the Amazon Resource Name (ARN) and resource ID format of the resource type for a specified IAM user, IAM role, or the root user for an account is affected. The opt-in and opt-out account setting must be set for each Amazon ECS resource separately. The ARN and resource ID format of a resource is defined by the opt-in status of the IAM user or role that created the resource. You must turn on this setting to use Amazon ECS features such as resource tagging.</p> <p>When <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for any new container instances that support the feature is changed. If <code>awsvpcTrunking</code> is enabled, any new container instances that support the feature are launched have the increased ENI limits available to them. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-eni.html">Elastic Network Interface Trunking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>When <code>containerInsights</code> is specified, the default setting indicating whether CloudWatch Container Insights is enabled for your clusters is changed. If <code>containerInsights</code> is enabled, any new clusters that are created will have Container Insights enabled unless you disable it during cluster creation. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cloudwatch-container-insights.html">CloudWatch Container Insights</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn put_account_setting(
        &self,
        input: PutAccountSettingRequest,
    ) -> Result<PutAccountSettingResponse, RusotoError<PutAccountSettingError>>;

    /// <p>Modifies an account setting for all IAM users on an account for whom no individual account setting has been specified. Account settings are set on a per-Region basis.</p>
    async fn put_account_setting_default(
        &self,
        input: PutAccountSettingDefaultRequest,
    ) -> Result<PutAccountSettingDefaultResponse, RusotoError<PutAccountSettingDefaultError>>;

    /// <p>Create or update an attribute on an Amazon ECS resource. If the attribute doesn't exist, it's created. If the attribute exists, its value is replaced with the specified value. To delete an attribute, use <a>DeleteAttributes</a>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes">Attributes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn put_attributes(
        &self,
        input: PutAttributesRequest,
    ) -> Result<PutAttributesResponse, RusotoError<PutAttributesError>>;

    /// <p>Modifies the available capacity providers and the default capacity provider strategy for a cluster.</p> <p>You must specify both the available capacity providers and a default capacity provider strategy for the cluster. If the specified cluster has existing capacity providers associated with it, you must specify all existing capacity providers in addition to any new ones you want to add. Any existing capacity providers that are associated with a cluster that are omitted from a <a>PutClusterCapacityProviders</a> API call will be disassociated with the cluster. You can only disassociate an existing capacity provider from a cluster if it's not being used by any existing tasks.</p> <p>When creating a service or running a task on a cluster, if no capacity provider or launch type is specified, then the cluster's default capacity provider strategy is used. We recommend that you define a default capacity provider strategy for your cluster. However, you must specify an empty array (<code>[]</code>) to bypass defining a default strategy.</p>
    async fn put_cluster_capacity_providers(
        &self,
        input: PutClusterCapacityProvidersRequest,
    ) -> Result<PutClusterCapacityProvidersResponse, RusotoError<PutClusterCapacityProvidersError>>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Registers an EC2 instance into the specified cluster. This instance becomes available to place containers on.</p></p>
    async fn register_container_instance(
        &self,
        input: RegisterContainerInstanceRequest,
    ) -> Result<RegisterContainerInstanceResponse, RusotoError<RegisterContainerInstanceError>>;

    /// <p>Registers a new task definition from the supplied <code>family</code> and <code>containerDefinitions</code>. Optionally, you can add data volumes to your containers with the <code>volumes</code> parameter. For more information about task definition parameters and defaults, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Amazon ECS Task Definitions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can specify an IAM role for your task with the <code>taskRoleArn</code> parameter. When you specify an IAM role for a task, its containers can then use the latest versions of the CLI or SDKs to make API requests to the Amazon Web Services services that are specified in the IAM policy that's associated with the role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">IAM Roles for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can specify a Docker networking mode for the containers in your task definition with the <code>networkMode</code> parameter. The available network modes correspond to those described in <a href="https://docs.docker.com/engine/reference/run/#/network-settings">Network settings</a> in the Docker run reference. If you specify the <code>awsvpc</code> network mode, the task is allocated an elastic network interface, and you must specify a <a>NetworkConfiguration</a> when you create a service or run a task with the task definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn register_task_definition(
        &self,
        input: RegisterTaskDefinitionRequest,
    ) -> Result<RegisterTaskDefinitionResponse, RusotoError<RegisterTaskDefinitionError>>;

    /// <p><p>Starts a new task using the specified task definition.</p> <p>You can allow Amazon ECS to place tasks for you, or you can customize how Amazon ECS places tasks using placement constraints and placement strategies. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html">Scheduling Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>Alternatively, you can use <a>StartTask</a> to use your own scheduler or place tasks manually on specific container instances.</p> <p>The Amazon ECS API follows an eventual consistency model. This is because of the distributed nature of the system supporting the API. This means that the result of an API command you run that affects your Amazon ECS resources might not be immediately visible to all subsequent commands you run. Keep this in mind when you carry out an API command that immediately follows a previous API command.</p> <p>To manage eventual consistency, you can do the following:</p> <ul> <li> <p>Confirm the state of the resource before you run a command to modify it. Run the DescribeTasks command using an exponential backoff algorithm to ensure that you allow enough time for the previous command to propagate through the system. To do this, run the DescribeTasks command repeatedly, starting with a couple of seconds of wait time and increasing gradually up to five minutes of wait time.</p> </li> <li> <p>Add wait time between subsequent commands, even if the DescribeTasks command returns an accurate response. Apply an exponential backoff algorithm starting with a couple of seconds of wait time, and increase gradually up to about five minutes of wait time.</p> </li> </ul></p>
    async fn run_task(
        &self,
        input: RunTaskRequest,
    ) -> Result<RunTaskResponse, RusotoError<RunTaskError>>;

    /// <p>Starts a new task from the specified task definition on the specified container instance or instances.</p> <p>Alternatively, you can use <a>RunTask</a> to place tasks for you. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html">Scheduling Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn start_task(
        &self,
        input: StartTaskRequest,
    ) -> Result<StartTaskResponse, RusotoError<StartTaskError>>;

    /// <p><p>Stops a running task. Any tags associated with the task will be deleted.</p> <p>When <a>StopTask</a> is called on a task, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> value and a default 30-second timeout, after which the <code>SIGKILL</code> value is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> value gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> value is sent.</p> <note> <p>The default 30-second timeout can be configured on the Amazon ECS container agent with the <code>ECS<em>CONTAINER</em>STOP_TIMEOUT</code> variable. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    async fn stop_task(
        &self,
        input: StopTaskRequest,
    ) -> Result<StopTaskResponse, RusotoError<StopTaskError>>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that an attachment changed states.</p></p>
    async fn submit_attachment_state_changes(
        &self,
        input: SubmitAttachmentStateChangesRequest,
    ) -> Result<SubmitAttachmentStateChangesResponse, RusotoError<SubmitAttachmentStateChangesError>>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a container changed states.</p></p>
    async fn submit_container_state_change(
        &self,
        input: SubmitContainerStateChangeRequest,
    ) -> Result<SubmitContainerStateChangeResponse, RusotoError<SubmitContainerStateChangeError>>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a task changed states.</p></p>
    async fn submit_task_state_change(
        &self,
        input: SubmitTaskStateChangeRequest,
    ) -> Result<SubmitTaskStateChangeResponse, RusotoError<SubmitTaskStateChangeError>>;

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags that are associated with that resource are deleted as well.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Modifies the parameters for a capacity provider.</p>
    async fn update_capacity_provider(
        &self,
        input: UpdateCapacityProviderRequest,
    ) -> Result<UpdateCapacityProviderResponse, RusotoError<UpdateCapacityProviderError>>;

    /// <p>Updates the cluster.</p>
    async fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> Result<UpdateClusterResponse, RusotoError<UpdateClusterError>>;

    /// <p>Modifies the settings to use for a cluster.</p>
    async fn update_cluster_settings(
        &self,
        input: UpdateClusterSettingsRequest,
    ) -> Result<UpdateClusterSettingsResponse, RusotoError<UpdateClusterSettingsError>>;

    /// <p>Updates the Amazon ECS container agent on a specified container instance. Updating the Amazon ECS container agent doesn't interrupt running tasks or services on the container instance. The process for updating the agent differs depending on whether your container instance was launched with the Amazon ECS-optimized AMI or another operating system.</p> <note> <p>The <code>UpdateContainerAgent</code> API isn't supported for container instances using the Amazon ECS-optimized Amazon Linux 2 (arm64) AMI. To update the container agent, you can update the <code>ecs-init</code> package. This updates the agent. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/agent-update-ecs-ami.html">Updating the Amazon ECS container agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note> <p>The <code>UpdateContainerAgent</code> API requires an Amazon ECS-optimized AMI or Amazon Linux AMI with the <code>ecs-init</code> service installed and running. For help updating the Amazon ECS container agent on other operating systems, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html#manually_update_agent">Manually updating the Amazon ECS container agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn update_container_agent(
        &self,
        input: UpdateContainerAgentRequest,
    ) -> Result<UpdateContainerAgentResponse, RusotoError<UpdateContainerAgentError>>;

    /// <p>Modifies the status of an Amazon ECS container instance.</p> <p>Once a container instance has reached an <code>ACTIVE</code> state, you can change the status of a container instance to <code>DRAINING</code> to manually remove an instance from a cluster, for example to perform system updates, update the Docker daemon, or scale down the cluster size.</p> <important> <p>A container instance can't be changed to <code>DRAINING</code> until it has reached an <code>ACTIVE</code> status. If the instance is in any other status, an error will be received.</p> </important> <p>When you set a container instance to <code>DRAINING</code>, Amazon ECS prevents new tasks from being scheduled for placement on the container instance and replacement service tasks are started on other container instances in the cluster if the resources are available. Service tasks on the container instance that are in the <code>PENDING</code> state are stopped immediately.</p> <p>Service tasks on the container instance that are in the <code>RUNNING</code> state are stopped and replaced according to the service's deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>. You can change the deployment configuration of your service using <a>UpdateService</a>.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during task replacement. For example, <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. If the minimum is 100%, the service scheduler can't remove existing tasks until the replacement tasks are considered healthy. Tasks for services that do not use a load balancer are considered healthy if they're in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they're in the <code>RUNNING</code> state and are reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during task replacement. You can use this to define the replacement batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four tasks to be drained, provided that the cluster resources required to do this are available. If the maximum is 100%, then replacement tasks can't start until the draining tasks have stopped.</p> </li> </ul> <p>Any <code>PENDING</code> or <code>RUNNING</code> tasks that do not belong to a service aren't affected. You must wait for them to finish or stop them manually.</p> <p>A container instance has completed draining when it has no more <code>RUNNING</code> tasks. You can verify this using <a>ListTasks</a>.</p> <p>When a container instance has been drained, you can set a container instance to <code>ACTIVE</code> status and once it has reached that status the Amazon ECS scheduler can begin scheduling tasks on the instance again.</p>
    async fn update_container_instances_state(
        &self,
        input: UpdateContainerInstancesStateRequest,
    ) -> Result<
        UpdateContainerInstancesStateResponse,
        RusotoError<UpdateContainerInstancesStateError>,
    >;

    /// <p><p>Modifies the parameters of a service.</p> <p>For services using the rolling update (<code>ECS</code>) you can update the desired count, deployment configuration, network configuration, load balancers, service registries, enable ECS managed tags option, propagate tags option, task placement constraints and strategies, and task definition. When you update any of these parameters, Amazon ECS starts new tasks with the new configuration. </p> <p>For services using the blue/green (<code>CODE<em>DEPLOY</code>) deployment controller, only the desired count, deployment configuration, health check grace period, task placement constraints and strategies, enable ECS managed tags option, and propagate tags can be updated using this API. If the network configuration, platform version, task definition, or load balancer need to be updated, create a new CodeDeploy deployment. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codedeploy/latest/APIReference/API</em>CreateDeployment.html&quot;&gt;CreateDeployment</a> in the <i>CodeDeploy API Reference</i>.</p> <p>For services using an external deployment controller, you can update only the desired count, task placement constraints and strategies, health check grace period, enable ECS managed tags option, and propagate tags option, using this API. If the launch type, load balancer, network configuration, platform version, or task definition need to be updated, create a new task set For more information, see <a>CreateTaskSet</a>.</p> <p>You can add to or subtract from the number of instantiations of a task definition in a service by specifying the cluster that the service is running in and a new <code>desiredCount</code> parameter.</p> <p>If you have updated the Docker image of your application, you can create a new task definition with that image and deploy it to your service. The service scheduler uses the minimum healthy percent and maximum percent parameters (in the service&#39;s deployment configuration) to determine the deployment strategy.</p> <note> <p>If your updated Docker image uses the same tag as what is in the existing task definition for your service (for example, <code>my<em>image:latest</code>), you don&#39;t need to create a new revision of your task definition. You can update the service using the <code>forceNewDeployment</code> option. The new tasks launched by the deployment pull the current image/tag combination from your repository when they start.</p> </note> <p>You can also update the deployment configuration of a service. When a deployment is triggered by updating the task definition of a service, the service scheduler uses the deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>, to determine the deployment strategy.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during a deployment. For example, if <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. Tasks for services that don&#39;t use a load balancer are considered healthy if they&#39;re in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they&#39;re in the <code>RUNNING</code> state and are reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during a deployment. You can use it to define the deployment batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available).</p> </li> </ul> <p>When <a>UpdateService</a> stops a task during a deployment, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> and a 30-second timeout. After this, <code>SIGKILL</code> is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> is sent.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster with the following logic.</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service&#39;s task definition. For example, they have the required CPU, memory, ports, and container instance attributes.</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner even though you can choose a different placement strategy.</p> <ul> <li> <p>Sort the valid container instances by the fewest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul> <p>When the service scheduler stops running tasks, it attempts to maintain balance across the Availability Zones in your cluster using the following logic: </p> <ul> <li> <p>Sort the container instances by the largest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have two, container instances in either zone B or C are considered optimal for termination.</p> </li> <li> <p>Stop the task on a container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the largest number of running tasks for this service.</p> </li> </ul> <note> <p>You must have a service-linked role when you update any of the following service properties. If you specified a custom IAM role when you created the service, Amazon ECS automatically replaces the &lt;a href=&quot;https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API</em>Service.html#ECS-Type-Service-roleArn&quot;&gt;roleARN</a> associated with the service with the ARN of your service-linked role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Service-linked roles</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <ul> <li> <p> <code>loadBalancers,</code> </p> </li> <li> <p> <code>serviceRegistries</code> </p> </li> </ul> </note></p>
    async fn update_service(
        &self,
        input: UpdateServiceRequest,
    ) -> Result<UpdateServiceResponse, RusotoError<UpdateServiceError>>;

    /// <p>Modifies which task set in a service is the primary task set. Any parameters that are updated on the primary task set in a service will transition to the service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn update_service_primary_task_set(
        &self,
        input: UpdateServicePrimaryTaskSetRequest,
    ) -> Result<UpdateServicePrimaryTaskSetResponse, RusotoError<UpdateServicePrimaryTaskSetError>>;

    /// <p>Modifies a task set. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn update_task_set(
        &self,
        input: UpdateTaskSetRequest,
    ) -> Result<UpdateTaskSetResponse, RusotoError<UpdateTaskSetError>>;
}
/// A client for the Amazon ECS API.
#[derive(Clone)]
pub struct EcsClient {
    client: Client,
    region: region::Region,
}

impl EcsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EcsClient {
        EcsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EcsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EcsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EcsClient {
        EcsClient { client, region }
    }
}

#[async_trait]
impl Ecs for EcsClient {
    /// <p>Creates a new capacity provider. Capacity providers are associated with an Amazon ECS cluster and are used in capacity provider strategies to facilitate cluster auto scaling.</p> <p>Only capacity providers that use an Auto Scaling group can be created. Amazon ECS tasks on Fargate use the <code>FARGATE</code> and <code>FARGATE_SPOT</code> capacity providers. These providers are available to all accounts in the Amazon Web Services Regions that Fargate supports.</p>
    async fn create_capacity_provider(
        &self,
        input: CreateCapacityProviderRequest,
    ) -> Result<CreateCapacityProviderResponse, RusotoError<CreateCapacityProviderError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.CreateCapacityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateCapacityProviderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateCapacityProviderResponse, _>()
    }

    /// <p><p>Creates a new Amazon ECS cluster. By default, your account receives a <code>default</code> cluster when you launch your first container instance. However, you can create your own cluster with a unique name with the <code>CreateCluster</code> action.</p> <note> <p>When you call the <a>CreateCluster</a> API operation, Amazon ECS attempts to create the Amazon ECS service-linked role for your account. This is so that it can manage required resources in other Amazon Web Services services on your behalf. However, if the IAM user that makes the call doesn&#39;t have permissions to create the service-linked role, it isn&#39;t created. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using service-linked roles for Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.CreateCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateClusterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateClusterResponse, _>()
    }

    /// <p><p>Runs and maintains your desired number of tasks from a specified task definition. If the number of tasks running in a service drops below the <code>desiredCount</code>, Amazon ECS runs another copy of the task in the specified cluster. To update an existing service, see the <a>UpdateService</a> action.</p> <p>In addition to maintaining the desired count of tasks in your service, you can optionally run your service behind one or more load balancers. The load balancers distribute traffic across the tasks that are associated with the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-load-balancing.html">Service load balancing</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>Tasks for services that don&#39;t use a load balancer are considered healthy if they&#39;re in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they&#39;re in the <code>RUNNING</code> state and are reported as healthy by the load balancer.</p> <p>There are two service scheduler strategies available:</p> <ul> <li> <p> <code>REPLICA</code> - The replica scheduling strategy places and maintains your desired number of tasks across your cluster. By default, the service scheduler spreads tasks across Availability Zones. You can use task placement strategies and constraints to customize task placement decisions. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Service scheduler concepts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li> <li> <p> <code>DAEMON</code> - The daemon scheduling strategy deploys exactly one task on each active container instance that meets all of the task placement constraints that you specify in your cluster. The service scheduler also evaluates the task placement constraints for running tasks. It also stops tasks that don&#39;t meet the placement constraints. When using this strategy, you don&#39;t need to specify a desired number of tasks, a task placement strategy, or use Service Auto Scaling policies. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Service scheduler concepts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li> </ul> <p>You can optionally specify a deployment configuration for your service. The deployment is initiated by changing properties. For example, the deployment might be initiated by the task definition or by your desired count of a service. This is done with an <a>UpdateService</a> operation. The default value for a replica service for <code>minimumHealthyPercent</code> is 100%. The default value for a daemon service for <code>minimumHealthyPercent</code> is 0%.</p> <p>If a service uses the <code>ECS</code> deployment controller, the minimum healthy percent represents a lower limit on the number of tasks in a service that must remain in the <code>RUNNING</code> state during a deployment. Specifically, it represents it as a percentage of your desired number of tasks (rounded up to the nearest integer). This happens when any of your container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. Using this parameter, you can deploy without using additional cluster capacity. For example, if you set your service to have desired number of four tasks and a minimum healthy percent of 50%, the scheduler might stop two existing tasks to free up cluster capacity before starting two new tasks. If they&#39;re in the <code>RUNNING</code> state, tasks for services that don&#39;t use a load balancer are considered healthy . If they&#39;re in the <code>RUNNING</code> state and reported as healthy by the load balancer, tasks for services that <i>do</i> use a load balancer are considered healthy . The default value for minimum healthy percent is 100%.</p> <p>If a service uses the <code>ECS</code> deployment controller, the <b>maximum percent</b> parameter represents an upper limit on the number of tasks in a service that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment. Specifically, it represents it as a percentage of the desired number of tasks (rounded down to the nearest integer). This happens when any of your container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. Using this parameter, you can define the deployment batch size. For example, if your service has a desired number of four tasks and a maximum percent value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default value for maximum percent is 200%.</p> <p>If a service uses either the <code>CODE_DEPLOY</code> or <code>EXTERNAL</code> deployment controller types and tasks that use the EC2 launch type, the <b>minimum healthy percent</b> and <b>maximum percent</b> values are used only to define the lower and upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state. This is while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the minimum healthy percent and maximum percent values aren&#39;t used. This is the case even if they&#39;re currently visible when describing your service.</p> <p>When creating a service that uses the <code>EXTERNAL</code> deployment controller, you can specify only parameters that aren&#39;t controlled at the task set level. The only required parameter is the service name. You control your services using the <a>CreateTaskSet</a> operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster using the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support the task definition of your service. For example, they have the required CPU, memory, ports, and container instance attributes.</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner. This is the case even if you can choose a different placement strategy with the <code>placementStrategy</code> parameter.</p> <ul> <li> <p>Sort the valid container instances, giving priority to instances that have the fewest number of running tasks for this service in their respective Availability Zone. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone based on the previous steps, favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul></p>
    async fn create_service(
        &self,
        input: CreateServiceRequest,
    ) -> Result<CreateServiceResponse, RusotoError<CreateServiceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.CreateService",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateServiceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateServiceResponse, _>()
    }

    /// <p>Create a task set in the specified cluster and service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn create_task_set(
        &self,
        input: CreateTaskSetRequest,
    ) -> Result<CreateTaskSetResponse, RusotoError<CreateTaskSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.CreateTaskSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTaskSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTaskSetResponse, _>()
    }

    /// <p>Disables an account setting for a specified IAM user, IAM role, or the root user for an account.</p>
    async fn delete_account_setting(
        &self,
        input: DeleteAccountSettingRequest,
    ) -> Result<DeleteAccountSettingResponse, RusotoError<DeleteAccountSettingError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteAccountSetting",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAccountSettingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteAccountSettingResponse, _>()
    }

    /// <p>Deletes one or more custom attributes from an Amazon ECS resource.</p>
    async fn delete_attributes(
        &self,
        input: DeleteAttributesRequest,
    ) -> Result<DeleteAttributesResponse, RusotoError<DeleteAttributesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteAttributesResponse, _>()
    }

    /// <p>Deletes the specified capacity provider.</p> <note> <p>The <code>FARGATE</code> and <code>FARGATE_SPOT</code> capacity providers are reserved and can't be deleted. You can disassociate them from a cluster using either the <a>PutClusterCapacityProviders</a> API or by deleting the cluster.</p> </note> <p>Prior to a capacity provider being deleted, the capacity provider must be removed from the capacity provider strategy from all services. The <a>UpdateService</a> API can be used to remove a capacity provider from a service's capacity provider strategy. When updating a service, the <code>forceNewDeployment</code> option can be used to ensure that any tasks using the Amazon EC2 instance capacity provided by the capacity provider are transitioned to use the capacity from the remaining capacity providers. Only capacity providers that aren't associated with a cluster can be deleted. To remove a capacity provider from a cluster, you can either use <a>PutClusterCapacityProviders</a> or delete the cluster.</p>
    async fn delete_capacity_provider(
        &self,
        input: DeleteCapacityProviderRequest,
    ) -> Result<DeleteCapacityProviderResponse, RusotoError<DeleteCapacityProviderError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteCapacityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteCapacityProviderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteCapacityProviderResponse, _>()
    }

    /// <p>Deletes the specified cluster. The cluster transitions to the <code>INACTIVE</code> state. Clusters with an <code>INACTIVE</code> status might remain discoverable in your account for a period of time. However, this behavior is subject to change in the future. We don't recommend that you rely on <code>INACTIVE</code> clusters persisting.</p> <p>You must deregister all container instances from this cluster before you may delete it. You can list the container instances in a cluster with <a>ListContainerInstances</a> and deregister them with <a>DeregisterContainerInstance</a>.</p>
    async fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, RusotoError<DeleteClusterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteClusterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteClusterResponse, _>()
    }

    /// <p><p>Deletes a specified service within a cluster. You can delete a service if you have no running tasks in it and the desired task count is zero. If the service is actively maintaining tasks, you can&#39;t delete it, and you must update the service to a desired task count of zero. For more information, see <a>UpdateService</a>.</p> <note> <p>When you delete a service, if there are still running tasks that require cleanup, the service status moves from <code>ACTIVE</code> to <code>DRAINING</code>, and the service is no longer visible in the console or in the <a>ListServices</a> API operation. After all tasks have transitioned to either <code>STOPPING</code> or <code>STOPPED</code> status, the service status moves from <code>DRAINING</code> to <code>INACTIVE</code>. Services in the <code>DRAINING</code> or <code>INACTIVE</code> status can still be viewed with the <a>DescribeServices</a> API operation. However, in the future, <code>INACTIVE</code> services may be cleaned up and purged from Amazon ECS record keeping, and <a>DescribeServices</a> calls on those services return a <code>ServiceNotFoundException</code> error.</p> </note> <important> <p>If you attempt to create a new service with the same name as an existing service in either <code>ACTIVE</code> or <code>DRAINING</code> status, you receive an error.</p> </important></p>
    async fn delete_service(
        &self,
        input: DeleteServiceRequest,
    ) -> Result<DeleteServiceResponse, RusotoError<DeleteServiceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteService",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteServiceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteServiceResponse, _>()
    }

    /// <p>Deletes a specified task set within a service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn delete_task_set(
        &self,
        input: DeleteTaskSetRequest,
    ) -> Result<DeleteTaskSetResponse, RusotoError<DeleteTaskSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteTaskSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTaskSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTaskSetResponse, _>()
    }

    /// <p><p>Deregisters an Amazon ECS container instance from the specified cluster. This instance is no longer available to run tasks.</p> <p>If you intend to use the container instance for some other purpose after deregistration, we recommend that you stop all of the tasks running on the container instance before deregistration. That prevents any orphaned tasks from consuming resources.</p> <p>Deregistering a container instance removes the instance from a cluster, but it doesn&#39;t terminate the EC2 instance. If you are finished using the instance, be sure to terminate it in the Amazon EC2 console to stop billing.</p> <note> <p>If you terminate a running container instance, Amazon ECS automatically deregisters the instance from your cluster (stopped container instances or instances with disconnected agents aren&#39;t automatically deregistered when terminated).</p> </note></p>
    async fn deregister_container_instance(
        &self,
        input: DeregisterContainerInstanceRequest,
    ) -> Result<DeregisterContainerInstanceResponse, RusotoError<DeregisterContainerInstanceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeregisterContainerInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeregisterContainerInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeregisterContainerInstanceResponse, _>()
    }

    /// <p><p>Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as <code>INACTIVE</code>. Existing tasks and services that reference an <code>INACTIVE</code> task definition continue to run without disruption. Existing services that reference an <code>INACTIVE</code> task definition can still scale up or down by modifying the service&#39;s desired count.</p> <p>You can&#39;t use an <code>INACTIVE</code> task definition to run new tasks or create new services, and you can&#39;t update an existing service to reference an <code>INACTIVE</code> task definition. However, there may be up to a 10-minute window following deregistration where these restrictions have not yet taken effect.</p> <note> <p>At this time, <code>INACTIVE</code> task definitions remain discoverable in your account indefinitely. However, this behavior is subject to change in the future. We don&#39;t recommend that you rely on <code>INACTIVE</code> task definitions persisting beyond the lifecycle of any associated tasks and services.</p> </note></p>
    async fn deregister_task_definition(
        &self,
        input: DeregisterTaskDefinitionRequest,
    ) -> Result<DeregisterTaskDefinitionResponse, RusotoError<DeregisterTaskDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeregisterTaskDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeregisterTaskDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeregisterTaskDefinitionResponse, _>()
    }

    /// <p>Describes one or more of your capacity providers.</p>
    async fn describe_capacity_providers(
        &self,
        input: DescribeCapacityProvidersRequest,
    ) -> Result<DescribeCapacityProvidersResponse, RusotoError<DescribeCapacityProvidersError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeCapacityProviders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCapacityProvidersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeCapacityProvidersResponse, _>()
    }

    /// <p>Describes one or more of your clusters.</p>
    async fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse, RusotoError<DescribeClustersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeClusters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeClustersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeClustersResponse, _>()
    }

    /// <p>Describes one or more container instances. Returns metadata about each container instance requested.</p>
    async fn describe_container_instances(
        &self,
        input: DescribeContainerInstancesRequest,
    ) -> Result<DescribeContainerInstancesResponse, RusotoError<DescribeContainerInstancesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeContainerInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeContainerInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeContainerInstancesResponse, _>()
    }

    /// <p>Describes the specified services running in your cluster.</p>
    async fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse, RusotoError<DescribeServicesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeServices",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeServicesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeServicesResponse, _>()
    }

    /// <p><p>Describes a task definition. You can specify a <code>family</code> and <code>revision</code> to find information about a specific task definition, or you can simply specify the family to find the latest <code>ACTIVE</code> revision in that family.</p> <note> <p>You can only describe <code>INACTIVE</code> task definitions while an active task or service references them.</p> </note></p>
    async fn describe_task_definition(
        &self,
        input: DescribeTaskDefinitionRequest,
    ) -> Result<DescribeTaskDefinitionResponse, RusotoError<DescribeTaskDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeTaskDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTaskDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeTaskDefinitionResponse, _>()
    }

    /// <p>Describes the task sets in the specified cluster and service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn describe_task_sets(
        &self,
        input: DescribeTaskSetsRequest,
    ) -> Result<DescribeTaskSetsResponse, RusotoError<DescribeTaskSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeTaskSets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTaskSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeTaskSetsResponse, _>()
    }

    /// <p>Describes a specified task or tasks.</p> <p>Currently, stopped tasks appear in the returned results for at least one hour.</p>
    async fn describe_tasks(
        &self,
        input: DescribeTasksRequest,
    ) -> Result<DescribeTasksResponse, RusotoError<DescribeTasksError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTasksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeTasksResponse, _>()
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Returns an endpoint for the Amazon ECS agent to poll for updates.</p></p>
    async fn discover_poll_endpoint(
        &self,
        input: DiscoverPollEndpointRequest,
    ) -> Result<DiscoverPollEndpointResponse, RusotoError<DiscoverPollEndpointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DiscoverPollEndpoint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DiscoverPollEndpointError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DiscoverPollEndpointResponse, _>()
    }

    /// <p>Runs a command remotely on a container within a task.</p> <p>If you use a condition key in your IAM policy to refine the conditions for the policy statement, for example limit the actions to a specific cluster, you recevie an <code>AccessDeniedException</code> when there is a mismatch between the condition key value and the corresponding parameter value.</p>
    async fn execute_command(
        &self,
        input: ExecuteCommandRequest,
    ) -> Result<ExecuteCommandResponse, RusotoError<ExecuteCommandError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ExecuteCommand",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ExecuteCommandError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ExecuteCommandResponse, _>()
    }

    /// <p>Lists the account settings for a specified principal.</p>
    async fn list_account_settings(
        &self,
        input: ListAccountSettingsRequest,
    ) -> Result<ListAccountSettingsResponse, RusotoError<ListAccountSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListAccountSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAccountSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAccountSettingsResponse, _>()
    }

    /// <p>Lists the attributes for Amazon ECS resources within a specified target type and cluster. When you specify a target type and cluster, <code>ListAttributes</code> returns a list of attribute objects, one for each attribute on each resource. You can filter the list of results to a single attribute name to only return results that have that name. You can also filter the results by attribute name and value. You can do this, for example, to see which container instances in a cluster are running a Linux AMI (<code>ecs.os-type=linux</code>). </p>
    async fn list_attributes(
        &self,
        input: ListAttributesRequest,
    ) -> Result<ListAttributesResponse, RusotoError<ListAttributesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAttributesResponse, _>()
    }

    /// <p>Returns a list of existing clusters.</p>
    async fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> Result<ListClustersResponse, RusotoError<ListClustersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListClusters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListClustersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListClustersResponse, _>()
    }

    /// <p>Returns a list of container instances in a specified cluster. You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements inside the <code>filter</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn list_container_instances(
        &self,
        input: ListContainerInstancesRequest,
    ) -> Result<ListContainerInstancesResponse, RusotoError<ListContainerInstancesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListContainerInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListContainerInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListContainerInstancesResponse, _>()
    }

    /// <p>Returns a list of services. You can filter the results by cluster, launch type, and scheduling strategy.</p>
    async fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> Result<ListServicesResponse, RusotoError<ListServicesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListServices",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListServicesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListServicesResponse, _>()
    }

    /// <p>List the tags for an Amazon ECS resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Returns a list of task definition families that are registered to your account. This list includes task definition families that no longer have any <code>ACTIVE</code> task definition revisions.</p> <p>You can filter out task definition families that don't contain any <code>ACTIVE</code> task definition revisions by setting the <code>status</code> parameter to <code>ACTIVE</code>. You can also filter the results with the <code>familyPrefix</code> parameter.</p>
    async fn list_task_definition_families(
        &self,
        input: ListTaskDefinitionFamiliesRequest,
    ) -> Result<ListTaskDefinitionFamiliesResponse, RusotoError<ListTaskDefinitionFamiliesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListTaskDefinitionFamilies",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTaskDefinitionFamiliesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListTaskDefinitionFamiliesResponse, _>()
    }

    /// <p>Returns a list of task definitions that are registered to your account. You can filter the results by family name with the <code>familyPrefix</code> parameter or by status with the <code>status</code> parameter.</p>
    async fn list_task_definitions(
        &self,
        input: ListTaskDefinitionsRequest,
    ) -> Result<ListTaskDefinitionsResponse, RusotoError<ListTaskDefinitionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListTaskDefinitions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTaskDefinitionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTaskDefinitionsResponse, _>()
    }

    /// <p>Returns a list of tasks. You can filter the results by cluster, task definition family, container instance, launch type, what IAM principal started the task, or by the desired status of the task.</p> <p>Recently stopped tasks might appear in the returned results. Currently, stopped tasks appear in the returned results for at least one hour.</p>
    async fn list_tasks(
        &self,
        input: ListTasksRequest,
    ) -> Result<ListTasksResponse, RusotoError<ListTasksError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTasksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTasksResponse, _>()
    }

    /// <p>Modifies an account setting. Account settings are set on a per-Region basis.</p> <p>If you change the account setting for the root user, the default settings for all of the IAM users and roles that no individual account setting was specified are reset for. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html">Account Settings</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>When <code>serviceLongArnFormat</code>, <code>taskLongArnFormat</code>, or <code>containerInstanceLongArnFormat</code> are specified, the Amazon Resource Name (ARN) and resource ID format of the resource type for a specified IAM user, IAM role, or the root user for an account is affected. The opt-in and opt-out account setting must be set for each Amazon ECS resource separately. The ARN and resource ID format of a resource is defined by the opt-in status of the IAM user or role that created the resource. You must turn on this setting to use Amazon ECS features such as resource tagging.</p> <p>When <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for any new container instances that support the feature is changed. If <code>awsvpcTrunking</code> is enabled, any new container instances that support the feature are launched have the increased ENI limits available to them. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-eni.html">Elastic Network Interface Trunking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>When <code>containerInsights</code> is specified, the default setting indicating whether CloudWatch Container Insights is enabled for your clusters is changed. If <code>containerInsights</code> is enabled, any new clusters that are created will have Container Insights enabled unless you disable it during cluster creation. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cloudwatch-container-insights.html">CloudWatch Container Insights</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn put_account_setting(
        &self,
        input: PutAccountSettingRequest,
    ) -> Result<PutAccountSettingResponse, RusotoError<PutAccountSettingError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.PutAccountSetting",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutAccountSettingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutAccountSettingResponse, _>()
    }

    /// <p>Modifies an account setting for all IAM users on an account for whom no individual account setting has been specified. Account settings are set on a per-Region basis.</p>
    async fn put_account_setting_default(
        &self,
        input: PutAccountSettingDefaultRequest,
    ) -> Result<PutAccountSettingDefaultResponse, RusotoError<PutAccountSettingDefaultError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.PutAccountSettingDefault",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutAccountSettingDefaultError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutAccountSettingDefaultResponse, _>()
    }

    /// <p>Create or update an attribute on an Amazon ECS resource. If the attribute doesn't exist, it's created. If the attribute exists, its value is replaced with the specified value. To delete an attribute, use <a>DeleteAttributes</a>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes">Attributes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn put_attributes(
        &self,
        input: PutAttributesRequest,
    ) -> Result<PutAttributesResponse, RusotoError<PutAttributesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.PutAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutAttributesResponse, _>()
    }

    /// <p>Modifies the available capacity providers and the default capacity provider strategy for a cluster.</p> <p>You must specify both the available capacity providers and a default capacity provider strategy for the cluster. If the specified cluster has existing capacity providers associated with it, you must specify all existing capacity providers in addition to any new ones you want to add. Any existing capacity providers that are associated with a cluster that are omitted from a <a>PutClusterCapacityProviders</a> API call will be disassociated with the cluster. You can only disassociate an existing capacity provider from a cluster if it's not being used by any existing tasks.</p> <p>When creating a service or running a task on a cluster, if no capacity provider or launch type is specified, then the cluster's default capacity provider strategy is used. We recommend that you define a default capacity provider strategy for your cluster. However, you must specify an empty array (<code>[]</code>) to bypass defining a default strategy.</p>
    async fn put_cluster_capacity_providers(
        &self,
        input: PutClusterCapacityProvidersRequest,
    ) -> Result<PutClusterCapacityProvidersResponse, RusotoError<PutClusterCapacityProvidersError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.PutClusterCapacityProviders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutClusterCapacityProvidersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutClusterCapacityProvidersResponse, _>()
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Registers an EC2 instance into the specified cluster. This instance becomes available to place containers on.</p></p>
    async fn register_container_instance(
        &self,
        input: RegisterContainerInstanceRequest,
    ) -> Result<RegisterContainerInstanceResponse, RusotoError<RegisterContainerInstanceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.RegisterContainerInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterContainerInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RegisterContainerInstanceResponse, _>()
    }

    /// <p>Registers a new task definition from the supplied <code>family</code> and <code>containerDefinitions</code>. Optionally, you can add data volumes to your containers with the <code>volumes</code> parameter. For more information about task definition parameters and defaults, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Amazon ECS Task Definitions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can specify an IAM role for your task with the <code>taskRoleArn</code> parameter. When you specify an IAM role for a task, its containers can then use the latest versions of the CLI or SDKs to make API requests to the Amazon Web Services services that are specified in the IAM policy that's associated with the role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">IAM Roles for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can specify a Docker networking mode for the containers in your task definition with the <code>networkMode</code> parameter. The available network modes correspond to those described in <a href="https://docs.docker.com/engine/reference/run/#/network-settings">Network settings</a> in the Docker run reference. If you specify the <code>awsvpc</code> network mode, the task is allocated an elastic network interface, and you must specify a <a>NetworkConfiguration</a> when you create a service or run a task with the task definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn register_task_definition(
        &self,
        input: RegisterTaskDefinitionRequest,
    ) -> Result<RegisterTaskDefinitionResponse, RusotoError<RegisterTaskDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.RegisterTaskDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterTaskDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RegisterTaskDefinitionResponse, _>()
    }

    /// <p><p>Starts a new task using the specified task definition.</p> <p>You can allow Amazon ECS to place tasks for you, or you can customize how Amazon ECS places tasks using placement constraints and placement strategies. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html">Scheduling Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>Alternatively, you can use <a>StartTask</a> to use your own scheduler or place tasks manually on specific container instances.</p> <p>The Amazon ECS API follows an eventual consistency model. This is because of the distributed nature of the system supporting the API. This means that the result of an API command you run that affects your Amazon ECS resources might not be immediately visible to all subsequent commands you run. Keep this in mind when you carry out an API command that immediately follows a previous API command.</p> <p>To manage eventual consistency, you can do the following:</p> <ul> <li> <p>Confirm the state of the resource before you run a command to modify it. Run the DescribeTasks command using an exponential backoff algorithm to ensure that you allow enough time for the previous command to propagate through the system. To do this, run the DescribeTasks command repeatedly, starting with a couple of seconds of wait time and increasing gradually up to five minutes of wait time.</p> </li> <li> <p>Add wait time between subsequent commands, even if the DescribeTasks command returns an accurate response. Apply an exponential backoff algorithm starting with a couple of seconds of wait time, and increase gradually up to about five minutes of wait time.</p> </li> </ul></p>
    async fn run_task(
        &self,
        input: RunTaskRequest,
    ) -> Result<RunTaskResponse, RusotoError<RunTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonEC2ContainerServiceV20141113.RunTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RunTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RunTaskResponse, _>()
    }

    /// <p>Starts a new task from the specified task definition on the specified container instance or instances.</p> <p>Alternatively, you can use <a>RunTask</a> to place tasks for you. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html">Scheduling Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn start_task(
        &self,
        input: StartTaskRequest,
    ) -> Result<StartTaskResponse, RusotoError<StartTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.StartTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartTaskResponse, _>()
    }

    /// <p><p>Stops a running task. Any tags associated with the task will be deleted.</p> <p>When <a>StopTask</a> is called on a task, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> value and a default 30-second timeout, after which the <code>SIGKILL</code> value is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> value gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> value is sent.</p> <note> <p>The default 30-second timeout can be configured on the Amazon ECS container agent with the <code>ECS<em>CONTAINER</em>STOP_TIMEOUT</code> variable. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    async fn stop_task(
        &self,
        input: StopTaskRequest,
    ) -> Result<StopTaskResponse, RusotoError<StopTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.StopTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopTaskResponse, _>()
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that an attachment changed states.</p></p>
    async fn submit_attachment_state_changes(
        &self,
        input: SubmitAttachmentStateChangesRequest,
    ) -> Result<SubmitAttachmentStateChangesResponse, RusotoError<SubmitAttachmentStateChangesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.SubmitAttachmentStateChanges",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SubmitAttachmentStateChangesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SubmitAttachmentStateChangesResponse, _>()
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a container changed states.</p></p>
    async fn submit_container_state_change(
        &self,
        input: SubmitContainerStateChangeRequest,
    ) -> Result<SubmitContainerStateChangeResponse, RusotoError<SubmitContainerStateChangeError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.SubmitContainerStateChange",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SubmitContainerStateChangeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SubmitContainerStateChangeResponse, _>()
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a task changed states.</p></p>
    async fn submit_task_state_change(
        &self,
        input: SubmitTaskStateChangeRequest,
    ) -> Result<SubmitTaskStateChangeResponse, RusotoError<SubmitTaskStateChangeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.SubmitTaskStateChange",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SubmitTaskStateChangeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SubmitTaskStateChangeResponse, _>()
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags that are associated with that resource are deleted as well.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.TagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UntagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Modifies the parameters for a capacity provider.</p>
    async fn update_capacity_provider(
        &self,
        input: UpdateCapacityProviderRequest,
    ) -> Result<UpdateCapacityProviderResponse, RusotoError<UpdateCapacityProviderError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateCapacityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateCapacityProviderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateCapacityProviderResponse, _>()
    }

    /// <p>Updates the cluster.</p>
    async fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> Result<UpdateClusterResponse, RusotoError<UpdateClusterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateClusterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateClusterResponse, _>()
    }

    /// <p>Modifies the settings to use for a cluster.</p>
    async fn update_cluster_settings(
        &self,
        input: UpdateClusterSettingsRequest,
    ) -> Result<UpdateClusterSettingsResponse, RusotoError<UpdateClusterSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateClusterSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateClusterSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateClusterSettingsResponse, _>()
    }

    /// <p>Updates the Amazon ECS container agent on a specified container instance. Updating the Amazon ECS container agent doesn't interrupt running tasks or services on the container instance. The process for updating the agent differs depending on whether your container instance was launched with the Amazon ECS-optimized AMI or another operating system.</p> <note> <p>The <code>UpdateContainerAgent</code> API isn't supported for container instances using the Amazon ECS-optimized Amazon Linux 2 (arm64) AMI. To update the container agent, you can update the <code>ecs-init</code> package. This updates the agent. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/agent-update-ecs-ami.html">Updating the Amazon ECS container agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note> <p>The <code>UpdateContainerAgent</code> API requires an Amazon ECS-optimized AMI or Amazon Linux AMI with the <code>ecs-init</code> service installed and running. For help updating the Amazon ECS container agent on other operating systems, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html#manually_update_agent">Manually updating the Amazon ECS container agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn update_container_agent(
        &self,
        input: UpdateContainerAgentRequest,
    ) -> Result<UpdateContainerAgentResponse, RusotoError<UpdateContainerAgentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateContainerAgent",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateContainerAgentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateContainerAgentResponse, _>()
    }

    /// <p>Modifies the status of an Amazon ECS container instance.</p> <p>Once a container instance has reached an <code>ACTIVE</code> state, you can change the status of a container instance to <code>DRAINING</code> to manually remove an instance from a cluster, for example to perform system updates, update the Docker daemon, or scale down the cluster size.</p> <important> <p>A container instance can't be changed to <code>DRAINING</code> until it has reached an <code>ACTIVE</code> status. If the instance is in any other status, an error will be received.</p> </important> <p>When you set a container instance to <code>DRAINING</code>, Amazon ECS prevents new tasks from being scheduled for placement on the container instance and replacement service tasks are started on other container instances in the cluster if the resources are available. Service tasks on the container instance that are in the <code>PENDING</code> state are stopped immediately.</p> <p>Service tasks on the container instance that are in the <code>RUNNING</code> state are stopped and replaced according to the service's deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>. You can change the deployment configuration of your service using <a>UpdateService</a>.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during task replacement. For example, <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. If the minimum is 100%, the service scheduler can't remove existing tasks until the replacement tasks are considered healthy. Tasks for services that do not use a load balancer are considered healthy if they're in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they're in the <code>RUNNING</code> state and are reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during task replacement. You can use this to define the replacement batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four tasks to be drained, provided that the cluster resources required to do this are available. If the maximum is 100%, then replacement tasks can't start until the draining tasks have stopped.</p> </li> </ul> <p>Any <code>PENDING</code> or <code>RUNNING</code> tasks that do not belong to a service aren't affected. You must wait for them to finish or stop them manually.</p> <p>A container instance has completed draining when it has no more <code>RUNNING</code> tasks. You can verify this using <a>ListTasks</a>.</p> <p>When a container instance has been drained, you can set a container instance to <code>ACTIVE</code> status and once it has reached that status the Amazon ECS scheduler can begin scheduling tasks on the instance again.</p>
    async fn update_container_instances_state(
        &self,
        input: UpdateContainerInstancesStateRequest,
    ) -> Result<
        UpdateContainerInstancesStateResponse,
        RusotoError<UpdateContainerInstancesStateError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateContainerInstancesState",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateContainerInstancesStateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateContainerInstancesStateResponse, _>()
    }

    /// <p><p>Modifies the parameters of a service.</p> <p>For services using the rolling update (<code>ECS</code>) you can update the desired count, deployment configuration, network configuration, load balancers, service registries, enable ECS managed tags option, propagate tags option, task placement constraints and strategies, and task definition. When you update any of these parameters, Amazon ECS starts new tasks with the new configuration. </p> <p>For services using the blue/green (<code>CODE<em>DEPLOY</code>) deployment controller, only the desired count, deployment configuration, health check grace period, task placement constraints and strategies, enable ECS managed tags option, and propagate tags can be updated using this API. If the network configuration, platform version, task definition, or load balancer need to be updated, create a new CodeDeploy deployment. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codedeploy/latest/APIReference/API</em>CreateDeployment.html&quot;&gt;CreateDeployment</a> in the <i>CodeDeploy API Reference</i>.</p> <p>For services using an external deployment controller, you can update only the desired count, task placement constraints and strategies, health check grace period, enable ECS managed tags option, and propagate tags option, using this API. If the launch type, load balancer, network configuration, platform version, or task definition need to be updated, create a new task set For more information, see <a>CreateTaskSet</a>.</p> <p>You can add to or subtract from the number of instantiations of a task definition in a service by specifying the cluster that the service is running in and a new <code>desiredCount</code> parameter.</p> <p>If you have updated the Docker image of your application, you can create a new task definition with that image and deploy it to your service. The service scheduler uses the minimum healthy percent and maximum percent parameters (in the service&#39;s deployment configuration) to determine the deployment strategy.</p> <note> <p>If your updated Docker image uses the same tag as what is in the existing task definition for your service (for example, <code>my<em>image:latest</code>), you don&#39;t need to create a new revision of your task definition. You can update the service using the <code>forceNewDeployment</code> option. The new tasks launched by the deployment pull the current image/tag combination from your repository when they start.</p> </note> <p>You can also update the deployment configuration of a service. When a deployment is triggered by updating the task definition of a service, the service scheduler uses the deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>, to determine the deployment strategy.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during a deployment. For example, if <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. Tasks for services that don&#39;t use a load balancer are considered healthy if they&#39;re in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they&#39;re in the <code>RUNNING</code> state and are reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during a deployment. You can use it to define the deployment batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available).</p> </li> </ul> <p>When <a>UpdateService</a> stops a task during a deployment, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> and a 30-second timeout. After this, <code>SIGKILL</code> is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> is sent.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster with the following logic.</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service&#39;s task definition. For example, they have the required CPU, memory, ports, and container instance attributes.</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner even though you can choose a different placement strategy.</p> <ul> <li> <p>Sort the valid container instances by the fewest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul> <p>When the service scheduler stops running tasks, it attempts to maintain balance across the Availability Zones in your cluster using the following logic: </p> <ul> <li> <p>Sort the container instances by the largest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have two, container instances in either zone B or C are considered optimal for termination.</p> </li> <li> <p>Stop the task on a container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the largest number of running tasks for this service.</p> </li> </ul> <note> <p>You must have a service-linked role when you update any of the following service properties. If you specified a custom IAM role when you created the service, Amazon ECS automatically replaces the &lt;a href=&quot;https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API</em>Service.html#ECS-Type-Service-roleArn&quot;&gt;roleARN</a> associated with the service with the ARN of your service-linked role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Service-linked roles</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <ul> <li> <p> <code>loadBalancers,</code> </p> </li> <li> <p> <code>serviceRegistries</code> </p> </li> </ul> </note></p>
    async fn update_service(
        &self,
        input: UpdateServiceRequest,
    ) -> Result<UpdateServiceResponse, RusotoError<UpdateServiceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateService",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateServiceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateServiceResponse, _>()
    }

    /// <p>Modifies which task set in a service is the primary task set. Any parameters that are updated on the primary task set in a service will transition to the service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn update_service_primary_task_set(
        &self,
        input: UpdateServicePrimaryTaskSetRequest,
    ) -> Result<UpdateServicePrimaryTaskSetResponse, RusotoError<UpdateServicePrimaryTaskSetError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateServicePrimaryTaskSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateServicePrimaryTaskSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateServicePrimaryTaskSetResponse, _>()
    }

    /// <p>Modifies a task set. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    async fn update_task_set(
        &self,
        input: UpdateTaskSetRequest,
    ) -> Result<UpdateTaskSetResponse, RusotoError<UpdateTaskSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateTaskSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateTaskSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateTaskSetResponse, _>()
    }
}
