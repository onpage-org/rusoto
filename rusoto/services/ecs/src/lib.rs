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
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p><fullname>Amazon Elastic Container Service</fullname> <p>Amazon Elastic Container Service (Amazon ECS) is a highly scalable, fast, container management service. It makes it easy to run, stop, and manage Docker containers. You can host your cluster on a serverless infrastructure that&#39;s managed by Amazon ECS by launching your services or tasks on Fargate. For more control, you can host your tasks on a cluster of Amazon Elastic Compute Cloud (Amazon EC2) or External (on-premises) instances that you manage.</p> <p>Amazon ECS makes it easy to launch and stop container-based applications with simple API calls. This makes it easy to get the state of your cluster from a centralized service, and gives you access to many familiar Amazon EC2 features.</p> <p>You can use Amazon ECS to schedule the placement of containers across your cluster based on your resource needs, isolation policies, and availability requirements. With Amazon ECS, you don&#39;t need to operate your own cluster management and configuration management systems. You also don&#39;t need to worry about scaling your management infrastructure.</p></p>
//!
//! If you're using the service, you're probably looking for [EcsClient](struct.EcsClient.html) and [Ecs](trait.Ecs.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
