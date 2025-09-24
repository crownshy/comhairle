# Comhairle

Comhairle is a platfrom that allows for consulation and deliberation at scale.

## Repo Structure 

- API: The main api server for orchestrating the platfrom 
- Frontend: The frontend code, currently a sveltekit application 
- data_model: The unified data model that describes our grammar of participation  
- adaptors: Adaptors for each of the tools we are integrating. These allow for setting up the tool, data extraction and login etc   

## Systems Architecture
Comhairle consists of a number of services that work together to deliver the full platform. Some of these services are ones that are built by CrownShy while others are open source tools that we run on our infrastructure.

### Owned services
**The API Server** handles all coordination of the system, between users, databases and open source tools. The API Serverserver is written in Rust using SQLX and Sea-Query talking to a postgresql database and axum as the web framework. 

Interactions with the frontend are handled by typical rest responses (though we may explore using gRPC at some point).

**The frontend**  is the web based interface where both citizens and policy makers will interact with the platfrom. is written in SvelteKit and uses Shadnn as a "Component Library" and Tailwind css for styling. Shadcn provides good defaults for accessibility and will allow us to rapidly build while still being able to customize.

In the future we may wrap the frontend using Tauri to provide a mobile app experience for the platform.

### Third party OS tools
Third party OS tools will be forked into the crownshy github organization. Deployment strategies for each will be developed for each and maintained as an “infrastructure as code” repo (see below for more details). 

The surface of these tools facing the web will be controlled through a  combination of proxies and ingress rules.

A data model library is specified in Rust and contains translators for each of the tools into a common data format.

## Infrastructure 
The system will be deployed to a K8 cluster, running on AWS EKS. The cluster itself will be managed by EKSCLT and the individual services will be managed by helm charts. Each tool will have it’s own helm chart which will be maintained in a private repo.
Modifications to the charts will be automatically taken live using github actions. 
Modifications to the owned services will similarly be taken live using a github action pipeline.

### Environments 
We will maintain two separate environments: Staging and Production. These will both run under different namespaces on the K8 cluster. One will track a staging branch on the monorepo / helm charts repo while the other will track tagged releases on the main branch.

## Authentication and Authorization
The system currently allows two types of users 
- Authenticated planners, who can create / edit / authorize conversation flows
- Anonymous citizens who can partake in the conversation flows
- 
While citizens are anonymous, they will be given a token that they can use to login at a future date / on a different device to continue to contribute. This token will be stored in local storage of the initiating device.

In the future we will aim to add a privacy preserving login like VeriFox or ScotAccount for citizens.
Passing Auth between the different services will be handled by JWT.
