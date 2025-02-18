# Comhairle

Comhairle is a platfrom that allows for consulation and deliberation at scale.

## Repo Structure 

- API: The main api server for orchestrating the platfrom 
- Frontend: The frontend code, currently a sveltekit application 
- data_model: The unified data model that describes our grammar of participation  
- adaptors: Adaptors for each of the tools we are integrating. These allow for setting up the tool, data extraction and login etc   

## Systems Archetecture 


The API server is ritten in Rust using [SQLX](https://github.com/launchbadge/sqlx) and [Sea-Query](https://github.com/SeaQL/sea-query) taking to a [postgresql](https://www.postgresql.org/) database and [axum](https://github.com/tokio-rs/axum) as the web framework. Interatctions with the front end 
are handled by typical rest responses (though we may explore using gRPC at some point). 

The frontend is written in SvelteKit and uses ShadCn as a "Component Library" and tailwind for styling 

The data model is specified in Rust and contains translators for each of the tools into a common data format. 

The deployed system works runs on a [K8](https://kubernetes.io/) cluster, running on [AWS EKS](https://aws.amazon.com/eks/) managed by [helm charts](https://helm.sh/). Each tool has it's own chart specifing how to set it 
up, scale it and run it. 


## Authentication and Autorization

The system currently allows two types of users 

1. Authenticated planners, who can create / edit / authorize conversation flows 
2. Anonymous citizens who can partake in the conversation flows

While citizens are anonymous, they will be given a token that they can use to login at a future date / on a different device to continue to contribute. This 
token will be stored in local storage of the initiating device.

In the future we will aim to add a privacy preserving login like [VeriFox](https://www.verifoxx.com/) or [ScotAccount](https://www.mygov.scot/scotaccount) for citizens. 

Passing Auth between the different services will be handled by JWT.

