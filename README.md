# Resumize (เรซูมั้ย?)

## Overview
This application helps generate resume content based on your experience and the job description of your interest.

![CleanShot 2567-04-28 at 01 25 21@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/b04e820e-68a5-4c15-b93c-c43aad035fe6)

## Tech Stack
- [Dioxus](https://dioxuslabs.com/) 0.5.1
- [Actix Web](https://actix.rs/) 4.5.1
- [Qdrant](https://qdrant.tech/) 1.9.0
- [LlamaEdge](https://github.com/LlamaEdge/LlamaEdge) with [Meta Llama 3](https://llama.meta.com/llama3/)

## Features
- **Generate Resume Content**: Input your dream job description and get a customized resume content. Please note that the result is not perfect.
- **Upload Experience**: Upload your past experiences in JSON format to be used for generating the resume content.

## Demo
[Link to your demo video]

## How it's designed

The project has been designed for few use-cases
1. Upload my own experience (as JSON)
2. Upload Job description from recruiter by copy from job market
3. Create Resume content based on job description
   with suggestion

![CleanShot 2567-04-28 at 01 25 38@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/eb8564e7-d3dc-46ac-a8d5-ca8745e2102b)

### Upload my own experience (as JSON)

![CleanShot 2567-04-28 at 01 26 00@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/3e222f35-80da-4b7a-ae17-d517545ed31c)

#### Description

TBC

#### Enhancement
- 1
- 2
- 3
- 4

### Upload job description and generate new resume content

![CleanShot 2567-04-30 at 13 47 35@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/f82b17db-a380-4d19-ba9b-42f28c393b88)

#### Description

TBC

#### Enhancement
- 1
- 2
- 3
- 4

## How to run locally

### Prerequisite
- [Git](https://git-scm.com/)
- [Install Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/products/docker-desktop/) or [Podman](https://podman.io/) or whatever you like e.g. [Colima](https://github.com/abiosoft/colima) to run Qdrant

### Make it run locally
- Clone the repository: `git clone https://github.com/marttp/rag-resume-generate`

### Run LlamaEdge for your local
2. Navigate to the project directory: `cd resume-generator`
3. Build the project: `cargo build --release`

### Run Qdrant

### Run Actix Web

### Run Dioxus

#### Locally

#### Bundle version

## License
This project is licensed under the MIT License.

## Contact
If you have any questions, feel free to reach out to me at below contact
- Facebook: [Thanaphoom Babparn](https://www.facebook.com/thanaphoom.mart/)
- FB Page: [TP Coder](https://www.facebook.com/tpcoder)
- LinkedIn: [Thanaphoom Babparn](https://www.linkedin.com/in/thanaphoom-babparn/)
- Website: [TP Coder — Portfolio](https://portfolio.tpcoder.dev/)
