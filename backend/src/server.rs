use crate::AppState;

use actix_web::{get, post, web, HttpResponse, Responder};
use aws_sdk_dynamodb::{config::IntoShared, types::AttributeValue};
use aws_sdk_ec2::types::{
    builders::IamInstanceProfileSpecificationBuilder, IamInstanceProfileSpecification, InstanceType,
};
use aws_sdk_ssm::types::error::InvalidInstanceId;
use serde::{Deserialize, Serialize};
use serde_dynamo::from_item;
use std::env;
use std::{collections::BTreeMap, fmt::format};
use tokio::time::{self, Duration};

#[derive(Deserialize)]
struct GetServersRequestData {
    user: String,
}

#[get("/servers")]
pub async fn list_servers(
    data: web::Data<AppState>,
    properties: web::Json<GetServersRequestData>,
) -> impl Responder {
    {
        let db = data.db_client.lock().unwrap();
    }

    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct CreateServerRequestData {
    user: String,
    server_name: String,
    data: std::collections::BTreeMap<String, String>,
}

#[derive(Serialize)]
struct CreateServerResponse {
    dns: String,
}

#[post("/server")]
pub async fn create_server(
    data: web::Data<AppState>,
    properties: web::Json<CreateServerRequestData>,
) -> impl Responder {
    let ec2_client = data.ec2_client.lock().unwrap();
    let ec2_res = ec2_client
        .run_instances()
        .min_count(1)
        .max_count(1)
        .instance_type(InstanceType::T2Small)
        .image_id("ami-008677ef1baf82eaf")
        .iam_instance_profile(
            IamInstanceProfileSpecification::builder()
                .set_arn(Some(format!(
                    "arn:aws:iam::{}:instance-profile/minecraft-server-instance",
                    env::var("ACCOUNT_ID").unwrap()
                )))
                .build(),
        )
        .security_group_ids(env::var("SERVER_INSTANCE_SEC_GRP").unwrap())
        .send()
        .await;

    let instance = match ec2_res {
        Ok(res) => {
            println!("ec2 res: {:?}", res);
            println!("id: {:?}", res.instances()[0].instance_id.clone().unwrap());

            res.instances()[0].clone()
        }
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError()
                .body(format!("Error trying to start ec2 instance: {:?}", e));
        }
    };

    let mut interval = time::interval(Duration::from_secs(25));
    'check: loop {
        tokio::select! {
            _ = interval.tick() => {
                let res = match ec2_client.describe_instances().instance_ids(instance.instance_id.clone().unwrap()).send().await {
                    Ok(res) => {
                        let instance_state = &res.reservations.unwrap()[0].instances()[0].state.clone().unwrap();

                        println!("instance state code: {:?}", instance_state.code.unwrap());
                        if instance_state.code.unwrap() == 16 {
                            break 'check;
                        }
                    },
                    Err(e) => {
                        println!("{:?}", e);
                    }
                };
            }
        }
    }

    {
        let ssm_client = data.ssm_client.lock().unwrap();

        let mut interval = time::interval(Duration::from_secs(45));
        'outer: loop {
            tokio::select! {
                _ = interval.tick() => {
                            let ssm_res = ssm_client
                                .send_command()
                                .instance_ids(instance.instance_id.clone().unwrap())
                                .document_name("AWS-RunShellScript".to_string())
                                .parameters("commands", vec![
                                    "TOKEN=$(aws ecr get-authorization-token --output text --query 'authorizationData[].authorizationToken' --region us-east-1)".to_string(), "sudo yum install docker -y".to_string(), "sudo systemctl start docker".to_string(), "sudo chmod 666 /var/run/docker.sock".to_string(), format!("aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin {}.dkr.ecr.us-east-1.amazonaws.com", env::var("ACCOUNT_ID").unwrap()), format!("docker pull {}.dkr.ecr.us-east-1.amazonaws.com/mine:server-img", env::var("ACCOUNT_ID").unwrap()), "rm -rf /home/ec2-user/.docker".to_string(), format!("docker run -p 8080:8080 -p25565:25565 {}.dkr.ecr.us-east-1.amazonaws.com/mine:server-img", env::var("ACCOUNT_ID").unwrap())])
                                .send()
                                .await;

                    let res = match ssm_res {
                        Ok(res) => {
                            println!("ssm res: {:?}", res);

                            let db_client = data.db_client.lock().unwrap();

                            break 'outer;
                        }
                        Err(sdk_error) => {
                            println!("ssm error: {:?}", sdk_error);

                            if sdk_error.as_service_error().map(|e| e.is_invalid_instance_id()) == Some(false) {
                                return HttpResponse::InternalServerError().body(format!(
                                    "Error while trying to run ssm document: {:?}",
                                    sdk_error
                                ));
                            }
                        }
                    };
                }
            }
        }
    }

    tokio::time::sleep(Duration::from_secs(60)).await;

    let dns = match ec2_client
        .describe_instances()
        .instance_ids(instance.instance_id.clone().unwrap())
        .send()
        .await
    {
        Ok(res) => {
            let dns = &res.reservations.unwrap()[0].instances()[0]
                .public_dns_name
                .clone()
                .unwrap();

            dns.to_string()
        }
        Err(e) => {
            println!("{:?}", e);

            "".to_string()
        }
    };

    println!("{}", dns);
    println!("{}", format!("http://{}:8080/start", dns));

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{}:8080/properties", dns))
        .json(&properties.data)
        .send()
        .await;
    println!("properties res: {:?}", resp);

    let resp = client
        .post(format!("http://{}:8080/start", dns))
        .send()
        .await;
    println!("start res: {:?}", resp);

    CreateServerResponse {
        dns: format!("{}", dns),
    };

    HttpResponse::Ok().body("Success!")
}
